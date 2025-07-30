using PassengerService.Infrastructure.Data.Models;

namespace PassengerService.Core.Interfaces;

public interface ITripJobRepository
{
    Task AddTripJobAsync(TripJob job);
    Task SaveChangesAsync();
}
