using FunTaxiMessagesProtoTrips;
using MassTransit;
using StackExchange.Redis;

public class TripStateMachine : MassTransitStateMachine<TripState>
{
    public State Requested { get; private set; }
    public State Accepted { get; private set; }
    public State Canceled { get; private set; }

    public Event<TripRequested> TripRequestedEvent { get; private set; }

    // public Event<TripAccepted> TripAcceptedEvent { get; private set; }

    // public Event<TripCanceled> TripCanceledEvent { get; private set; }

    public TripStateMachine()
    {
        InstanceState(x => x.CurrentState);

        Event(
            () => TripRequestedEvent,
            x => x.CorrelateById(context => Guid.Parse(context.Message.Id))
        );
        // Event(() => TripAcceptedEvent, x => x.CorrelateById(context => context.Message.Id));

        Initially(
            When(TripRequestedEvent)
                .Then(context =>
                {
                    context.Saga.UserId = Guid.Parse(context.Message.UserId);
                    context.Saga.RequestedDate = DateTime.UtcNow;
                })
                // NOTE: temp solution for prototyping
                // migrate to geo service?
                .ThenAsync(
                    async (context) =>
                    {
                        var db = context
                            .GetServiceOrCreateInstance<IConnectionMultiplexer>()
                            .GetDatabase();
                        var tran = db.CreateTransaction();
                        _ = tran.GeoAddAsync(
                            "available_trips:start",
                            context.Message.StartLon,
                            context.Message.StartLat,
                            context.Message.Id
                        );
                        _ = tran.GeoAddAsync(
                            "available_trips:end",
                            context.Message.EndLon,
                            context.Message.EndLat,
                            context.Message.Id
                        );
                        await tran.ExecuteAsync();
                    }
                )
                .TransitionTo(Requested)
        );

        // During(
        //     Requested,
        //     When(TripAcceptedEvent)
        //         .Then(context =>
        //         {
        //             context.Instance.DriverId = context.Message.DriverId;
        //             context.Instance.AcceptedDate = DateTime.UtcNow;
        //         })
        //         .TransitionTo(Accepted)
        //         .Finalize()
        // );

        // During(
        //     Accepted,
        //     When(TripCanceledEvent)
        //         .Then(context =>
        //         {
        //             Console.WriteLine(
        //                 "ignore event, send user error that drive is in progress (for now)"
        //             );
        //         })
        // );

        // During(
        //     Requested,
        //     When(TripCanceledEvent)
        //         .Then(context =>
        //         {
        //             Console.WriteLine(
        //                 "// remove trip info from db, drivers would not see the event"
        //             );
        //         })
        //         .TransitionTo(Canceled)
        //         .Finalize()
        // );

        // During(
        //     Canceled,
        //     When(TripAcceptedEvent)
        //         .Then(context =>
        //         {
        //             Console.WriteLine(" // send driver notification that trip is cancelled");
        //         })
        // );

        SetCompletedWhenFinalized();
    }
}
