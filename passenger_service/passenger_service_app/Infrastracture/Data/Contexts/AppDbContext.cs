using Microsoft.EntityFrameworkCore;
using PassengerService.Infrastructure.Data.Models;

namespace PassengerService.Infrastructure.Data;

public class AppDbContext : DbContext
{
    public AppDbContext(DbContextOptions<AppDbContext> options)
        : base(options) { }

    public DbSet<TripJob> Jobs { get; set; }
}
