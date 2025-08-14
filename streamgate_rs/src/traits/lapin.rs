use lapin::{Channel, ExchangeKind, options::*, types::FieldTable};
use tonic::async_trait;

#[async_trait]
pub trait DeclareAndBindFanoutExchange {
    async fn declare_and_bind_fanout_exchange(
        &self,
        queue: &str,
        exchange_name: &str,
    ) -> Result<(), lapin::Error>;
}

#[async_trait]
impl DeclareAndBindFanoutExchange for Channel {
    async fn declare_and_bind_fanout_exchange(
        &self,
        queue: &str,
        exchange_name: &str,
    ) -> Result<(), lapin::Error> {
        self.exchange_declare(
            exchange_name,
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        ).await?;

        self.queue_bind(
            queue,
            exchange_name,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await?;

        Ok(())
    }
}
