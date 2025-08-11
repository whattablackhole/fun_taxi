using System.Text.Json;
using FluentAssertions;
using FunTaxi.Messages.Trips.V1;
using PactNet;
using PactNet.Matchers;
using PactNet.Output.Xunit;
using Xunit.Abstractions;

public class PassengerTripEventConsumerTests
{
    private readonly IMessagePactBuilderV4 messagePact;

    public PassengerTripEventConsumerTests(ITestOutputHelper output)
    {
        var config = new PactConfig
        {
            PactDir = "../../../pacts",
            Outputters = [new XunitOutput(output)],
            LogLevel = PactLogLevel.Debug,
            DefaultJsonSettings = { PropertyNamingPolicy = JsonNamingPolicy.CamelCase },
        };

        this.messagePact = Pact.V4(
                "Passenger Trip Event Consumer",
                "Passenger Trip Event Producer",
                config
            )
            .WithMessageInteractions();
    }

    [Fact]
    public void ReceiveTripRequestEvents()
    {
        this.messagePact.ExpectsToReceive("some trip request events")
            .WithJsonContent(
                Match.MinType(
                    new
                    {
                        Id = Match.Type("UUID"),
                        EndLat = Match.Type(1.23d),
                        EndLon = Match.Type(1.23d),
                        StartLat = Match.Type(1.23d),
                        StartLon = Match.Type(1.23d),
                        UserId = Match.Type("UUID"),
                    },
                    1
                )
            )
            .Verify<ICollection<PassengerTripRequested>>(events =>
            {
                events.Should().NotBeNullOrEmpty();
                events.Should().HaveCount(1);

                events
                    .First()
                    .Should()
                    .BeEquivalentTo(
                        new PassengerTripRequested
                        {
                            EndLat = 1.23d,
                            EndLon = 1.23d,
                            StartLat = 1.23d,
                            StartLon = 1.23d,
                            Id = "UUID",
                            UserId = "UUID",
                        }
                    );
            });
    }
}
