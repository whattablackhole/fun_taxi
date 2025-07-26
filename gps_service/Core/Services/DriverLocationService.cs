using GPS_Service.Core.Interfaces;

namespace GPS_Service.Core.Services;

internal class DriverLocationService : IDriverLocationService
{
    readonly IMessageBus _MessageBus;

    public DriverLocationService(IMessageBus messageBus)
    {
        _MessageBus = messageBus;
    }

    public async Task UpdateDriverLocationChangeAsync(UpdateDriverPosition updateDriverLocation)
    {
        await _MessageBus.PublishAsync(
            "gps_driver_position",
            updateDriverLocation.driverId,
            updateDriverLocation
        );
    }
}
