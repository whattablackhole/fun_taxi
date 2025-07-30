using PassengerService.Core.Interfaces;
using PassengerService.Infrastructure.Data.Models;

namespace PassengerService.Infrastructure.Data.Repositories;

public class TripJobRepository : ITripJobRepository
{
    readonly AppDbContext _dbContext;

    public TripJobRepository(AppDbContext context)
    {
        _dbContext = context;
    }

    public async Task AddTripJobAsync(TripJob job)
    {
        _dbContext.Attach(job);
        await _dbContext.AddAsync(job);
    }

    public async Task SaveChangesAsync()
    {
        await _dbContext.SaveChangesAsync();
    }
}
