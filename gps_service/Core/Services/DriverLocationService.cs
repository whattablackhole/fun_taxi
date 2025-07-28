using FunTaxi.Messages.Proto.Gps;
using GPS_Service.Core.Interfaces;

namespace GPS_Service.Core.Services;

internal class DriverLocationService : IDriverLocationService
{
    readonly IProtobufMessageBusProducer _messageBus;

    public DriverLocationService(IProtobufMessageBusProducer messageBus)
    {
        _messageBus = messageBus;
    }

    public async Task UpdateDriverLocationChangeAsync(DriverPosition updateDriverLocation)
    {
        try
        {
            await _messageBus.ProduceAsync(
                "gps_driver_position",
                updateDriverLocation.DriverId,
                new UpdateDriverPositionPayload
                {
                    DriverId = updateDriverLocation.DriverId,
                    Location =
                    {
                        Latitude = updateDriverLocation.Location.Lat,
                        Longitude = updateDriverLocation.Location.Lon,
                    },
                }
            );
        }
        catch
        {
            // TODO
        }
    }
}
