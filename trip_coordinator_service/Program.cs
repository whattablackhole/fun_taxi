using MassTransit;
using StackExchange.Redis;
using TripCoordinatorService.Infrastructure.MessageBrockers;

var builder = WebApplication.CreateBuilder(args);

// TEMP
builder.Services.AddSingleton<IConnectionMultiplexer>(sp =>
{
    var configuration = builder.Configuration.GetConnectionString("Redis")!;
    return ConnectionMultiplexer.Connect(configuration);
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

app.Run();
