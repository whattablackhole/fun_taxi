using FunTaxi.Messages.Proto.Gps;
using GPS_Service.Core.Interfaces;

namespace GPS_Service.Core.Services;

internal class DriverLocationService : IDriverLocationService
{
    readonly IProtobufMessageBusProducer _messageBus;
    readonly ILogger<IDriverLocationService> _logger;

    public DriverLocationService(
        IProtobufMessageBusProducer messageBus,
        ILogger<IDriverLocationService> logger
    )
    {
        _messageBus = messageBus;
        _logger = logger;
    }

    public async Task UpdateDriverLocationChangeAsync(DriverPosition updateDriverLocation)
    {
        _logger.LogDebug(
            "Start producing new DriverPosition: {@DriverMessage}",
            updateDriverLocation
        );
        try
        {
            await _messageBus.ProduceAsync(
                "gps_driver_position",
                updateDriverLocation.DriverId,
                new UpdateDriverPositionPayload
                {
                    DriverId = updateDriverLocation.DriverId,
                    Location = new GeoLocation
                    {
                        Latitude = updateDriverLocation.Location.Lat,
                        Longitude = updateDriverLocation.Location.Lon,
                    },
                }
            );
        }
        catch (Exception ex)
        {
            _logger.LogWarning(
                "Exception occured while producing gps_driver_position {@Exception}",
                ex
            );
            throw;
        }
    }
}
