using Confluent.Kafka;
using GPS_Service.Api.Hubs;
using GPS_Service.Core.Interfaces;
using GPS_Service.Core.Services;
using GPS_Service.Infrastracture;
using Serilog;
using Serilog.Events;

var builder = WebApplication.CreateBuilder(args);

Log.Logger = new LoggerConfiguration()
    .MinimumLevel.Debug()
    .MinimumLevel.Override("Microsoft", LogEventLevel.Warning)
    .MinimumLevel.Override("System", LogEventLevel.Warning)
    .Enrich.FromLogContext()
    .WriteTo.Console(
        outputTemplate: "[{Timestamp:HH:mm:ss} {Level:u3}] {Message:lj}{NewLine}{Exception}"
    )
    .WriteTo.File(
        path: "Logs/log-.txt",
        rollingInterval: RollingInterval.Day,
        outputTemplate: "{Timestamp:yyyy-MM-dd HH:mm:ss.fff zzz} [{Level:u3}] {Message:lj}{NewLine}{Exception}"
    )
    .CreateLogger();

try
{
    builder.Host.UseSerilog();
    builder.Services.AddSignalR();
    builder.Services.AddSingleton<IDriverLocationService, DriverLocationService>();
    builder.Services.AddSingleton<IMessageBus>(provider =>
    {
        var config = new ProducerConfig
        {
            BootstrapServers = builder.Configuration["KafkaBootstrapServers"],
        };
        // NOTE: kafka's connection is not tested before first injection
        // TODO: add check
        return new KafkaMessageBus(config);
    });
    var app = builder.Build();
    app.MapHub<DriversHub>("/driversHub");

    Console.WriteLine("Application Started...");

    app.Run();
}
catch (Exception ex)
{
    Log.Fatal(ex, "Application terminated unexpectedly");
}
finally
{
    Log.CloseAndFlush();
}
