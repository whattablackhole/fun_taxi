using System.ComponentModel.DataAnnotations;

public class NearestDriversDto
{
    [Required]
    public double Lat { get; set; }

    [Required]
    public double Lon { get; set; }

    [Required]
    public double Radius { get; set; }
}

public class DriverAndPositionDto
{
    public required double Lat { get; set; }

    public required double Lon { get; set; }

    public required string DriverId { get; set; }
}

public class NearestDriversResponseDto
{
    public IEnumerable<DriverAndPositionDto> driversAndPositions { get; set; } = [];
}
