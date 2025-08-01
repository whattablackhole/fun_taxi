using MassTransit;
using Microsoft.AspNetCore.Server.Kestrel.Core;
using StackExchange.Redis;
using TripCoordinatorService.Infrastructure.MessageBrockers;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddGrpc();

// TEMP
builder.Services.AddSingleton<IConnectionMultiplexer>(sp =>
{
    var configuration = builder.Configuration.GetConnectionString("Redis")!;
    return ConnectionMultiplexer.Connect(configuration);
});

builder.WebHost.ConfigureKestrel(options =>
{
    options.ListenAnyIP(
        int.Parse(builder.Configuration["GRPC_PORT"]!),
        listenOptions =>
        {
            listenOptions.Protocols = HttpProtocols.Http2;
            // listenOptions.UseHttps();
        }
    );
});

builder.Services.AddMassTransit(
    (c) =>
    {
        c.UsingRabbitMq(
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
                cfg.UseRawJsonSerializer();
                cfg.ConfigureEndpoints(context);
            }
        );
        c.AddSagaStateMachine<TripStateMachine, TripState>().InMemoryRepository();
    }
);
var app = builder.Build();

app.MapGrpcService<TripsFinderServiceImpl>();

app.Run();
