using System.Text.Json;
using FunTaxi.Messages.Trips.V1;
using PactNet.Infrastructure.Outputters;
using PactNet.Verifier;
using Xunit.Abstractions;

namespace passenger_service_tests.Contracts;

public class StockEventGeneratorTests : IDisposable
{
    private readonly PactVerifier verifier;

    public StockEventGeneratorTests(ITestOutputHelper output)
    {
        var config = new PactVerifierConfig
        {
            LogLevel = PactNet.PactLogLevel.Debug,
            Outputters = new List<IOutput> { new XUnitOutput(output) },
        };

        this.verifier = new PactVerifier("Passenger Trip Event Producer", config);
    }

    public void Dispose()
    {
        GC.SuppressFinalize(this);
        this.verifier.Dispose();
    }

    [Fact]
    public void EnsureEventApiHonoursPactWithConsumer()
    {
        string pactPath = Path.Combine(
            "..",
            "..",
            "..",
            "..",
            "..",
            "trip_coordinator_service",
            "trip_coordinator_service.tests",
            "pacts",
            "Passenger Trip Event Consumer-Passenger Trip Event Producer.json"
        );

        var defaultSettings = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        };
        this.verifier.WithHttpEndpoint(new Uri("http://localhost:5000"));

        this.verifier.WithMessages(
                scenarios =>
                {
                    scenarios.Add(
                        "some trip request events",
                        builder =>
                            builder.WithContent(() =>
                            {
                                return new List<PassengerTripRequested>
                                {
                                    new PassengerTripRequested
                                    {
                                        Id = Guid.NewGuid().ToString(),
                                        EndLat = 34.0522,
                                        EndLon = -118.2437,
                                        StartLat = 34.0000,
                                        StartLon = -118.0000,
                                        UserId = Guid.NewGuid().ToString(),
                                    },
                                };
                            })
                    );
                },
                defaultSettings
            )
            .WithFileSource(new FileInfo(pactPath))
            .Verify();
    }
}

public class XUnitOutput : IOutput
{
    private readonly ITestOutputHelper _output;

    public XUnitOutput(ITestOutputHelper output)
    {
        _output = output;
    }

    public void WriteLine(string line)
    {
        _output.WriteLine(line);
    }
}
