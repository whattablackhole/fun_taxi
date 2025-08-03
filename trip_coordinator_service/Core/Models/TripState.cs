using MassTransit;

public class TripState : SagaStateMachineInstance
{
    public Guid CorrelationId { get; set; }

    public string CurrentState { get; set; }

    public Guid UserId { get; set; }

    public Guid DriverId { get; set; }

    public double StartLat { get; set; }
    public double StartLon { get; set; }

    public double EndLat { get; set; }
    public double EndLon { get; set; }

    public DateTime? RequestedDate { get; set; }

    public DateTime? AcceptedDate { get; set; }
}
