using FunTaxiMessagesProtoTrips;
using Grpc.Core;
using StackExchange.Redis;

public class TripsFinderServiceImpl : TripsFinderService.TripsFinderServiceBase
{
    private readonly IDatabase _db;
    private readonly ILogger<TripsFinderServiceImpl> _logger;

    public TripsFinderServiceImpl(
        IConnectionMultiplexer connectionMultiplexer,
        ILogger<TripsFinderServiceImpl> logger
    )
    {
        _db = connectionMultiplexer.GetDatabase();
        _logger = logger;
    }

    public override async Task<AvailableTripsReply> GetAvailableTrips(
        AvailableTripsRequest request,
        ServerCallContext context
    )
    {
        var reply = new AvailableTripsReply();

        _logger.LogDebug("Getting available trips from db... {@request}", request);

        // TODO: Optimize with Lua script for atomic query of start + end
        var radiusResults = await _db.GeoSearchAsync(
            "available_trips:start",
            request.Lon,
            request.Lat,
            new GeoSearchCircle(request.Radius, GeoUnit.Kilometers)
        );

        _logger.LogDebug("available_trips:start: {@results}", radiusResults);

        if (radiusResults == null || radiusResults.Length == 0)
        {
            return reply;
        }

        var tripIds = radiusResults.Select(x => x.Member).ToArray();
        var destinationPositions = await _db.GeoPositionAsync("available_trips:end", tripIds);

        _logger.LogDebug("destinationPositions: {@positions}", destinationPositions);

        if (destinationPositions == null || destinationPositions.Length != radiusResults.Length)
        {
            _logger.LogWarning("mismatch between start and end entries: {@results}", radiusResults);
            return reply;
        }

        for (var i = 0; i < radiusResults.Length; i++)
        {
            var start = radiusResults[i].Position;
            var end = destinationPositions[i];

            if (!start.HasValue || !end.HasValue)
                continue;

            reply.Trips.Add(
                new Trip
                {
                    Id = radiusResults[i].Member,
                    StartLat = start.Value.Latitude,
                    StartLon = start.Value.Longitude,
                    EndLat = end.Value.Latitude,
                    EndLon = end.Value.Longitude,
                }
            );
        }

        return reply;
    }
}
