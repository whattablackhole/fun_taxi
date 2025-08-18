using FunTaxi.Messages.Trips.V1;
using MassTransit;
using StackExchange.Redis;

public class TripStateMachine : MassTransitStateMachine<TripState>
{
    public State Requested { get; private set; }
    public State Accepted { get; private set; }
    public State Cancelled { get; private set; }
    public State InProgress { get; private set; }
    public State Completed { get; private set; }

    public Event<PassengerTripRequested> PassengerTripRequestedEvent { get; private set; }

    public Event<DriverTripAccepted> TripAcceptedEvent { get; private set; }

    public Event<TripCancellationRequested> TripCancellationRequestedEvent { get; private set; }

    public TripStateMachine()
    {
        InstanceState(x => x.CurrentState);

        Event(
            () => PassengerTripRequestedEvent,
            x => x.CorrelateById(context => Guid.Parse(context.Message.Id))
        );
        Event(
            () => TripAcceptedEvent,
            x => x.CorrelateById(context => Guid.Parse(context.Message.Id))
        );

        Event(
            () => TripCancellationRequestedEvent,
            x => x.CorrelateById(context => Guid.Parse(context.Message.Id))
        );

        Initially(
            When(PassengerTripRequestedEvent)
                .Then(context =>
                {
                    context.Saga.UserId = Guid.Parse(context.Message.UserId);
                    context.Saga.RequestedDate = DateTime.UtcNow;
                    context.Saga.EndLat = context.Message.EndLat;
                    context.Saga.EndLon = context.Message.EndLon;
                    context.Saga.StartLat = context.Message.StartLat;
                    context.Saga.StartLon = context.Message.StartLon;
                })
                .Publish(
                    (ctx) =>
                    {
                        return new TripGeoPositionAddCommand
                        {
                            Id = ctx.Message.Id,
                            StartLat = ctx.Saga.StartLat,
                            StartLon = ctx.Saga.StartLon,
                            EndLat = ctx.Saga.EndLat,
                            EndLon = ctx.Saga.EndLon,
                        };
                    }
                )
                .TransitionTo(Requested)
        );

        During(
            Requested,
            When(TripAcceptedEvent)
                .Then(context =>
                {
                    context.Saga.DriverId = Guid.Parse(context.Message.DriverId);
                    context.Saga.AcceptedDate = DateTime.UtcNow;
                })
                .Publish(
                    (ctx) =>
                    {
                        return new TripGeoPositionRemoveCommand()
                        {
                            Id = ctx.Saga.CorrelationId.ToString(),
                        };
                    }
                )
                // Add Respond for fast acknowledge?
                .Publish(
                    (ctx) =>
                    {
                        return new TripAssignedToDriver()
                        {
                            Id = ctx.Saga.CorrelationId.ToString(),
                            DriverId = ctx.Saga.DriverId.ToString(),
                        };
                    },
                    (publishCtx) =>
                    {
                        publishCtx.Headers.Set("x-user-id", publishCtx.Message.DriverId);
                    }
                )
                .TransitionTo(Accepted)
        );

        During(
            Requested,
            When(TripCancellationRequestedEvent)
                // TODO: add check if user elible to cancel
                .Publish(
                    (ctx) =>
                    {
                        return new TripGeoPositionRemoveCommand()
                        {
                            Id = ctx.Saga.CorrelationId.ToString(),
                        };
                    }
                )
                .Publish(
                    (ctx) =>
                    {
                        return new TripCancelledByUser()
                        {
                            Id = ctx.Saga.CorrelationId.ToString(),
                            UserId = ctx.Message.UserId,
                        };
                    }
                )
                .TransitionTo(Cancelled)
        );

        During(
            Accepted,
            When(TripCancellationRequestedEvent)
                .Publish(
                    (ctx) =>
                    {
                        return new TripCancelledByUser()
                        {
                            Id = ctx.Saga.CorrelationId.ToString(),
                            UserId = ctx.Message.UserId,
                        };
                    }
                )
                .TransitionTo(Cancelled)
        );

        During(
            Cancelled,
            When(TripAcceptedEvent)
                .Publish(ctx => new TripAcceptionRejection() { Id = ctx.CorrelationId.ToString() })
        );

        During(
            Cancelled,
            When(TripCancellationRequestedEvent)
                .Publish(ctx => new TripCancellationRejection()
                {
                    Id = ctx.CorrelationId.ToString(),
                })
        );

        SetCompletedWhenFinalized();
    }
}
