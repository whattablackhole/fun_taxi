namespace GPS_Service.Core.Interfaces;

public interface IMessageBus
{
    public Task PublishAsync<T>(string topic, string key, T message)
        where T : class;
}
