using MassTransit;

public class TripState : SagaStateMachineInstance
{
    public Guid CorrelationId { get; set; }

    public string CurrentState { get; set; }

    public Guid UserId { get; set; }

    public Guid DriverId { get; set; }

    public DateTime? RequestedDate { get; set; }

    public DateTime? AcceptedDate { get; set; }
}
