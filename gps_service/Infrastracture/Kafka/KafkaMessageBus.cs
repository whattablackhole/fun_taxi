using System.Text.Json;
using Confluent.Kafka;
using GPS_Service.Core.Interfaces;

namespace GPS_Service.Infrastracture;

internal class KafkaMessageBus : IMessageBus
{
    private readonly IProducer<string, string> _Producer;

    public KafkaMessageBus(ProducerConfig producerConfig)
    {
        _Producer = new ProducerBuilder<string, string>(producerConfig).Build();
    }

    public async Task PublishAsync<T>(string topic, string key, T message)
        where T : class
    {
        await _Producer.ProduceAsync(
            topic,
            new Message<string, string> { Value = JsonSerializer.Serialize(message), Key = key }
        );
    }
}
