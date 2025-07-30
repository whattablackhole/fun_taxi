using AutoMapper;
using MassTransit;
using MassTransit.Testing;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using PassengerService.Core.Domain.Models;
using PassengerService.Core.Domain.Services;
using PassengerService.Infrastructure.Data;
using PassengerService.Infrastructure.Data.Repositories;
using PassengerService.Infrastructure.MessageBrockers.Models;
using PassengerService.Mapping;

namespace passenger_service_tests.Services;

public class TripSearchServiceTests : IAsyncLifetime
{
    private InMemoryTestHarness _harness;
    private AppDbContext _dbContext;

    public async Task InitializeAsync()
    {
        _harness = new InMemoryTestHarness();
        await _harness.Start();

        var options = new DbContextOptionsBuilder<AppDbContext>()
            .UseSqlite("Filename=:memory:")
            .Options;

        _dbContext = new AppDbContext(options);
        _dbContext.Database.OpenConnection();
        _dbContext.Database.EnsureCreated();
    }

    public async Task DisposeAsync()
    {
        await _harness.Stop();
        await _dbContext.DisposeAsync();
    }

    [Fact]
    public async Task ProcessNewTripSearch_ShouldCreateTripJobAndPublishMessage()
    {
        IPublishEndpoint publishEndpoint = _harness.Bus;

        var repo = new TripJobRepository(_dbContext);

        var config = new MapperConfiguration(
            cfg =>
            {
                cfg.AddProfile<MappingProfile>();
            },
            new LoggerFactory()
        );
        var mapper = config.CreateMapper();

        var service = new TripSearchService(repo, publishEndpoint, mapper);

        var trip = new CreateTripSearchJob
        {
            UserId = Guid.NewGuid(),
            TripStartLocation = new GeoLocation(1.0, 1.0),
            TripEndLocation = new GeoLocation(2.0, 2.0),
        };

        // Act
        await service.ProcessNewTripSearch(trip);

        // Assert
        Assert.Equal(1, _dbContext.Jobs.Count());
        Assert.True(await _harness.Published.Any<TripSearchRequestedMessage>());
    }
}
