using GPS_Service.Models;

namespace GPS_Service.Core.Interfaces;

public interface IDriverLocationService
{
    public Task UpdateDriverLocationChangeAsync(UpdateDriverPosition updateDriverPosition);
}
