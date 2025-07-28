using Confluent.Kafka;
using Confluent.SchemaRegistry;
using Confluent.SchemaRegistry.Serdes;
using Google.Protobuf;
using GPS_Service.Core.Interfaces;

namespace GPS_Service.Infrastructure;

internal class KafkaMessageBus : IProtobufMessageBusProducer, IDisposable
{
    private readonly IProducer<string, byte[]> _producer;
    private readonly ISchemaRegistryClient _schemaRegistryClient;
    private readonly ILogger<KafkaMessageBus> _logger;
    private bool _disposed;

    public KafkaMessageBus(
        IProducer<string, byte[]> producer,
        ISchemaRegistryClient schemaRegistryClient,
        ILogger<KafkaMessageBus> logger
    )
    {
        _producer = producer ?? throw new ArgumentNullException(nameof(producer));
        _schemaRegistryClient =
            schemaRegistryClient ?? throw new ArgumentNullException(nameof(schemaRegistryClient));
        _logger = logger ?? throw new ArgumentNullException(nameof(logger));

        _logger.LogInformation("KafkaMessageBus initialized.");
    }

    public Task ProduceAsync<T>(string topic, string key, T message)
        where T : IMessage<T>, new()
    {
        throw new NotImplementedException();
    }

    public async Task ProduceAsync<T>(
        string topic,
        string key,
        T value,
        CancellationToken cancellationToken = default
    )
        where T : IMessage<T>, new()
    {
        if (_disposed)
        {
            throw new ObjectDisposedException(
                nameof(KafkaMessageBus),
                "Cannot produce messages after the bus has been disposed."
            );
        }

        try
        {
            var protobufSerializer = new ProtobufSerializer<T>(_schemaRegistryClient);

            var serializedValue = await protobufSerializer.SerializeAsync(
                value,
                new SerializationContext(MessageComponentType.Value, topic)
            );

            var message = new Message<string, byte[]> { Key = key, Value = serializedValue };

            var deliveryResult = await _producer.ProduceAsync(topic, message, cancellationToken);
        }
        catch (OperationCanceledException)
        {
            _logger.LogWarning("Produce operation to topic '{Topic}' was cancelled.", topic);
            throw;
        }
        catch (ProduceException<string, byte[]> ex)
        {
            _logger.LogError(
                ex,
                "Delivery failed for message '{Key}' to topic '{Topic}': {Reason}",
                key,
                topic,
                ex.Error.Reason
            );
            throw;
        }
        catch (Exception ex)
        {
            _logger.LogError(
                ex,
                "An unexpected error occurred while producing message '{Key}' to topic '{Topic}'.",
                key,
                topic
            );
            throw;
        }
    }

    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }

    protected virtual void Dispose(bool disposing)
    {
        if (!_disposed)
        {
            if (disposing)
            {
                _logger.LogInformation("Disposing Kafka producer and Schema Registry client.");
                _producer.Dispose();
                _schemaRegistryClient.Dispose();
            }

            _disposed = true;
        }
    }

    ~KafkaMessageBus()
    {
        Dispose(disposing: false);
    }
}
