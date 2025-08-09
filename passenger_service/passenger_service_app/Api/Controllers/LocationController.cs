using AutoMapper;
using FunTaxi.Messages.Gps.V1;
using Microsoft.AspNetCore.Mvc;
using Prometheus;

namespace PassengerService.Api.Controllers;

[ApiController]
[Route("location")]
public class LocationController : ControllerBase
{
    readonly DriversFinderService.DriversFinderServiceClient _driverFinderService;
    readonly IMapper _mapper;

    private static readonly Counter RequestCounter = Metrics.CreateCounter(
        "nearest_drivers_requests_total",
        "Number of requests for nearest drivers"
    );

    private static readonly Histogram GrpcDuration = Metrics.CreateHistogram(
        "grpc_search_drivers_duration_seconds",
        "Time taken to call SearchDriversByPositionAsync"
    );

    public LocationController(
        DriversFinderService.DriversFinderServiceClient driverFinderService,
        IMapper mapper
    )
    {
        _driverFinderService = driverFinderService;
        _mapper = mapper;
    }

    [HttpPost("nearest_drivers")]
    public async Task<ActionResult> NearestDrivers(NearestDriversDto payload)
    {
        try
        {
            RequestCounter.Inc();

            var request = new SearchDriversRequest
            {
                Lat = payload.Lat,
                Lon = payload.Lon,
                Radius = payload.Radius,
            };

            using (GrpcDuration.NewTimer())
            {
                var reply = await _driverFinderService.SearchDriversByPositionAsync(request);

                var response = new NearestDriversResponseDto
                {
                    driversAndPositions = reply.DriversAndPositions.Select(
                        d => new DriverAndPositionDto
                        {
                            DriverId = d.DriverId,
                            Lat = d.Lat,
                            Lon = d.Lon,
                        }
                    ),
                };

                return Ok(response);
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine("Unhandled Exception {@ex}", ex);

            return BadRequest();
        }
    }
}
