use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    // fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
    //     info!("Pre rebalance {:?}", rebalance);
    // }

    // fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
    //     info!("Post rebalance {:?}", rebalance);
    // }

    // fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
    //     info!("Committing offsets: {:?}", result);
    // }
}
type GeopositionConsumer = StreamConsumer<CustomContext>;
pub async fn consume_geoposition(brokers: &str, group_id: &str, topics: &[&str]) {
    let context: CustomContext = CustomContext;
  
    let consumer: GeopositionConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");
    consumer
    .subscribe(&topics.to_vec())
    .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                // let payload = match m.payload_view::<str>() {
                //     None => "",
                //     Some(Ok(s)) => s,
                //     Some(Err(e)) => {
                //         warn!("Error while deserializing message payload: {:?}", e);
                //         ""
                //     }
                // };
                // info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                //       m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                // if let Some(headers) = m.headers() {
                //     for header in headers.iter() {
                //         info!("  Header {:#?}: {:?}", header.key, header.value);
                //     }
                // }
                // consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}