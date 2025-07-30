namespace PassengerService.Infrastructure.MessageBrockers.Models;

public class TripSearchRequestedMessage
{
    public Guid Id { get; set; }
    public double StartLat { get; set; }
    public double StartLon { get; set; }
    public double EndLat { get; set; }
    public double EndLon { get; set; }
    public Guid UserId { get; set; }
}
