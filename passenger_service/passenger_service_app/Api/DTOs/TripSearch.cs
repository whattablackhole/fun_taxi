using System.ComponentModel.DataAnnotations;

namespace PassengerService.Api.DTOs;

public class LocationDTO
{
    [Required]
    public double Lat { get; set; }

    [Required]
    public double Lon { get; set; }
}

public class CreateTripSearchDTO
{
    [Required]
    public Guid UserId { get; set; }

    [Required]
    public LocationDTO? TripStartLocation { get; set; }

    [Required]
    public LocationDTO? TripEndLocation { get; set; }
}
