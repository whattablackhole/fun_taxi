using GPS_Service.Api.Messages;
using GPS_Service.Core.Interfaces;
using Microsoft.AspNetCore.SignalR;

namespace GPS_Service.Api.Hubs;

public class DriversHub : Hub
{
    readonly IDriverLocationService _driverLocationService;
    readonly ILogger<DriversHub> _logger;

    public DriversHub(IDriverLocationService driverLocationService, ILogger<DriversHub> logger)
    {
        _driverLocationService = driverLocationService;
        _logger = logger;
    }

    public async Task SendMessage(string user, DriverBaseMessage message)
    {
        switch (message.Type)
        {
            case DriverMessageType.LocationChange:
            {
                var msg = (DriverLocationChangedMessage)message;
                await _driverLocationService.UpdateDriverLocationChangeAsync(
                    new UpdateDriverPosition { driverId = user, location = msg.Location }
                );
                break;
            }
            default:
            {
                _logger.LogWarning($"Unexpected message from driver: {message.Type} {user}");
                break;
            }
        }
    }

    public override Task OnConnectedAsync()
    {
        Console.WriteLine($"Client connected: {Context.ConnectionId}");
        return base.OnConnectedAsync();
    }

    public override Task OnDisconnectedAsync(Exception? exception)
    {
        Console.WriteLine($"Client disconnected: {Context.ConnectionId}");
        return base.OnDisconnectedAsync(exception);
    }
}
