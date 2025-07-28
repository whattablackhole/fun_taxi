using GPS_Service.Models;

public class DriverPosition()
{
    public string DriverId { get; set; } = string.Empty;

    public GeoLocation Location { get; set; }
}
