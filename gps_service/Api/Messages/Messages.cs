using GPS_Service.Models;

namespace GPS_Service.Api.Messages;

public enum DriverMessageType
{
    LocationChange,
}

public abstract class DriverBaseMessage
{
    public DriverMessageType Type { get; set; }
}

public class DriverLocationChangedMessage : DriverBaseMessage
{
    public GeoLocation Location { get; set; }
    public DateTime SentAt { get; set; } = DateTime.UtcNow;
}

public class DriverSystemMessage : DriverBaseMessage
{
    public string Info { get; set; } = string.Empty;
}
