using AutoMapper;
using FunTaxi.Messages.Trips.V1;
using MassTransit;
using PassengerService.Core.Domain.Models;
using PassengerService.Core.Interfaces;
using PassengerService.Infrastructure.Data.Models;

namespace PassengerService.Core.Domain.Services;

public class TripSearchService(
    ITripJobRepository jobRepository,
    IPublishEndpoint publishEndpoint,
    IMapper mapper
)
{
    private readonly ITripJobRepository _jobRepository = jobRepository;
    private readonly IPublishEndpoint _publishEndpoint = publishEndpoint;

    private readonly IMapper _mapper = mapper;

    public async Task ProcessNewTripSearch(CreateTripSearchJob job)
    {
        var tripJob = new TripJob
        {
            EndLat = job.TripEndLocation.Lat,
            EndLon = job.TripEndLocation.Lon,
            StartLat = job.TripStartLocation.Lat,
            StartLon = job.TripStartLocation.Lon,
            State = TripJobState.Pending,
            UserId = job.UserId,
        };

        await _jobRepository.AddTripJobAsync(tripJob);
        await _jobRepository.SaveChangesAsync();

        var message = _mapper.Map<PassengerTripRequested>(tripJob);

        await _publishEndpoint.Publish(message);
    }
}
