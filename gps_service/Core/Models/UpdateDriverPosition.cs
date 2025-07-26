using GPS_Service.Models;

public class UpdateDriverPosition()
{
    public string driverId { get; set; } = string.Empty;

    public GeoLocation location { get; set; }
}
