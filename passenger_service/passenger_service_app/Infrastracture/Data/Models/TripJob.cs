using System.ComponentModel.DataAnnotations;
using Microsoft.EntityFrameworkCore;

namespace PassengerService.Infrastructure.Data.Models;

public enum TripJobState
{
    Pending,
    Cancelled,
    Completed,
}

public class TripJob
{
    [Key]
    public Guid Id { get; set; }

    [Required]
    public Guid UserId { get; set; } = default!;

    public TripJobState State { get; set; } = TripJobState.Pending;

    [Required]
    public double StartLat { get; set; } = default!;

    [Required]
    public double StartLon { get; set; } = default!;

    [Required]
    public double EndLat { get; set; } = default!;

    [Required]
    public double EndLon { get; set; } = default!;
}
