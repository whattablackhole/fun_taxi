using Google.Protobuf;

namespace GPS_Service.Core.Interfaces;

public interface IMessageBusProducer
{
    public Task ProduceAsync<T>(string topic, string key, T message);
}

public interface IProtobufMessageBusProducer
{
    Task ProduceAsync<T>(
        string topic,
        string key,
        T message,
        CancellationToken cancellationToken = default
    )
        where T : IMessage<T>, new();
}
