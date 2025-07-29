using System.Text.Json;
using GPS_Service.Api.Messages;
using GPS_Service.Core.Interfaces;
using Microsoft.AspNetCore.SignalR;

namespace GPS_Service.Api.Hubs;

public class DriversHub : Hub
{
    readonly IDriverLocationService _driverLocationService;
    readonly ILogger<DriversHub> _logger;

    public DriversHub(ILogger<DriversHub> logger, IDriverLocationService driverLocationService)
    {
        _driverLocationService = driverLocationService;
        _logger = logger;
    }

    public async Task SendMessage(DriverMessage message)
    {
        _logger.LogDebug("Received DriverMessage: {@DriverMessage}", message);

        switch (message.Type)
        {
            case DriverMessageType.LocationChange:
                // TODO: introduce try catch send error.
                var payload = message.Payload.Deserialize<DriverLocationChangedMessage>();
                _logger.LogDebug(
                    "Processing LocationChange DriverMessage: {@DriverMessage}",
                    payload
                );
                await _driverLocationService.UpdateDriverLocationChangeAsync(
                    new DriverPosition { DriverId = message.DriverId, Location = payload.Location }
                );
                break;
            default:
            {
                _logger.LogWarning(
                    $"Unexpected message from driver: {message.Type}, {message.DriverId}"
                );
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
