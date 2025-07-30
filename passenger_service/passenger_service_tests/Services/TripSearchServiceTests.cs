using Moq;
using PassengerService.Core.Domain.Models;
using PassengerService.Core.Domain.Services;
using PassengerService.Core.Interfaces;
using PassengerService.Infrastructure.Data.Models;
using PassengerService.Infrastructure.MessageBrockers.Models;

namespace passenger_service_tests.Services;

public class TripSearchServiceTests
{
    [Fact]
    public async Task ProcessNewTripSearch_ShouldPublishJobMessageCorrectly()
    {
        // Arrange
        var message = new TripSearchRequestedMessage
        {
            EndLat = 2.0,
            EndLon = 2.0,
            Id = Guid.NewGuid(),
            StartLat = 1.0,
            StartLon = 1.0,
            UserId = Guid.NewGuid(),
        };
        var mockRepository = new Mock<ITripJobRepository>();
        var mockMapper = new Mock<AutoMapper.IMapper>();
        mockMapper
            .Setup(m => m.Map<TripSearchRequestedMessage>(It.IsAny<TripJob>()))
            .Returns(message);
        var mockBus = new Mock<MassTransit.IPublishEndpoint>();
        var service = new TripSearchService(
            mockRepository.Object,
            mockBus.Object,
            mockMapper.Object
        );
        var trip = new CreateTripSearchJob
        {
            UserId = Guid.NewGuid(),
            TripStartLocation = new GeoLocation(1.0, 1.0),
            TripEndLocation = new GeoLocation(2.0, 2.0),
        };

        // Act
        await service.ProcessNewTripSearch(trip);

        // Assert
        mockBus.Verify(m => m.Publish(message, It.IsAny<CancellationToken>()), Times.Once);
    }
}
