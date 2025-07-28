namespace GPS_Service.Core.Interfaces;

public interface IDriverLocationService
{
    public Task UpdateDriverLocationChangeAsync(DriverPosition updateDriverPosition);
}
