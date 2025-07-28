using Confluent.Kafka;
using Confluent.SchemaRegistry;
using GPS_Service.Api.Hubs;
using GPS_Service.Core.Interfaces;
using GPS_Service.Core.Services;
using GPS_Service.Infrastructure;
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

    var producerConfig = new ProducerConfig
    {
        BootstrapServers = builder.Configuration["KafkaBootstrapServers"],
    };
    builder.Services.AddSingleton(producerConfig);

    var schemaRegistryConfig = new SchemaRegistryConfig
    {
        Url = builder.Configuration["SchemaRegistryUrl"],
    };

    builder.Services.AddSingleton(schemaRegistryConfig);

    builder.Services.AddSingleton(sp =>
    {
        var config = sp.GetRequiredService<ProducerConfig>();
        return new ProducerBuilder<string, byte[]>(config)
            .SetErrorHandler(
                (_, e) =>
                    sp.GetRequiredService<ILogger<IProducer<string, byte[]>>>()
                        .LogError($"Kafka Error: {e.Reason}")
            )
            .Build();
    });

    builder.Services.AddSingleton<ISchemaRegistryClient>(sp =>
    {
        var config = sp.GetRequiredService<SchemaRegistryConfig>();
        return new CachedSchemaRegistryClient(config);
    });

    builder.Services.AddSingleton<IProtobufMessageBusProducer, KafkaMessageBus>();

    builder.Services.AddSingleton<IDriverLocationService, DriverLocationService>();

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
