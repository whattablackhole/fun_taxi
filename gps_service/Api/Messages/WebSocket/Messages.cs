using System.Text.Json;
using GPS_Service.Models;

namespace GPS_Service.Api.Messages;

public enum DriverMessageType
{
    LocationChange,
}

public class DriverMessage
{
    public DriverMessageType Type { get; set; }
    public string DriverId { get; set; }
    public JsonElement Payload { get; set; }
}

public class DriverLocationChangedMessage
{
    public GeoLocation Location { get; set; }
    public DateTime SentAt { get; set; } = DateTime.UtcNow;
}

public class DriverSystemMessage
{
    public string Info { get; set; } = string.Empty;
}
