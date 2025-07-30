namespace PassengerService.Core.Domain.Models;

public enum TripSearchJobState
{
    Pending,
    Canceled,
    Completed,
}

public class CreateTripSearchJob
{
    public Guid UserId { get; set; } = default!;
    public GeoLocation TripStartLocation { get; set; } = default!;

    public GeoLocation TripEndLocation { get; set; } = default!;
}
