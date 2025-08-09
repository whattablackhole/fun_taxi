using AutoMapper;
using FunTaxi.Messages.Gps.V1;
using MassTransit;
using Microsoft.EntityFrameworkCore;
using PassengerService.Core.Domain.Services;
using PassengerService.Core.Interfaces;
using PassengerService.Infrastructure.Data;
using PassengerService.Infrastructure.Data.Repositories;
using PassengerService.Infrastructure.MessageBrockers;
using PassengerService.Mapping;
using Prometheus;

var builder = WebApplication.CreateBuilder(args);
var appConfig = builder.Configuration;
builder.Services.AddControllers();
builder.Services.AddAutoMapper(
    cfg =>
    {
        cfg.LicenseKey = appConfig["AutoMapperLicense"];
    },
    typeof(MappingProfile)
);

builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseSqlite("Data Source=passenger.db")
);

builder.Services.AddGrpcClient<DriversFinderService.DriversFinderServiceClient>(o =>
{
    o.Address = new Uri(appConfig["GEO_SERVICE_GRPC_ADDRESS"]);
});

builder.Services.AddMassTransit(x =>
{
    x.UsingRabbitMq(
        (context, cfg) =>
        {
            var rabbitSettings = builder
                .Configuration.GetSection("RabbitMQ")
                .Get<RabbitMQSetting>();
            cfg.Host(
                rabbitSettings!.Host,
                (c) =>
                {
                    c.Username(rabbitSettings.UserName!);
                    c.Password(rabbitSettings.Password!);
                }
            );
        }
    );
});

builder.Services.AddScoped<ITripJobRepository, TripJobRepository>();

builder.Services.AddScoped<TripSearchService>();

var app = builder.Build();

#if DEBUG
var lf = app.Services.GetRequiredService<ILoggerFactory>();
var config = new MapperConfiguration(
    cfg =>
    {
        cfg.LicenseKey = appConfig["AutoMapperLicense"];
        cfg.AddProfile<MappingProfile>();
    },
    lf
);

try
{
    config.AssertConfigurationIsValid();
}
catch (AggregateException ex)
{
    foreach (AutoMapperConfigurationException inner in ex.InnerExceptions)
    {
        Console.WriteLine(inner.Message);
    }
    throw;
}
#endif

app.UseHttpMetrics();
app.MapMetrics();
app.UseHttpsRedirection();
app.MapControllers();

app.Run();
