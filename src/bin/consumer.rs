#[macro_use]
extern crate log;
extern crate clap;
extern crate rdkafka;
extern crate tokio_core;
extern crate futures;

mod utils;

use rdkafka::consumer::{Consumer, ConsumerContext};
use futures::stream::Stream;

use clap::{App, Arg};
use tokio_core::reactor::Core;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::client::Context;
use rdkafka::config::{ClientConfig, TopicConfig};
use rdkafka::error::KafkaError;
use rdkafka::message::Message;

pub struct LocalContext;
impl Context for LocalContext {
    fn error(&self, error: KafkaError, reason: &str) {
        error!(">>>>>>>>>>>>>>>>> librdkafka: {}: {}", error, reason);
    }
}
impl ConsumerContext for LocalContext {}

fn run_consumer(brokers: &str, group_id: &str, topic: &str) {
    // Create the event loop. The event loop will run on a single thread and drive the pipeline.
    let mut core = Core::new().unwrap();

    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    let consumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set_default_topic_config(TopicConfig::new()
            // .set("auto.offset.reset", "smallest")
            .finalize())
        .create_with_context::<_, StreamConsumer<_>>(LocalContext {})
        .expect("Consumer creation failed");

    consumer.subscribe(&[topic]).expect("Can't subscribe to specified topic");

    // Create the outer pipeline on the message stream.
    let processed_stream = consumer.start()
        .filter_map(|result| {
            // Filter out errors
            match result {
                Ok(msg) => Some(msg),
                Err(kafka_error) => {
                    println!("Error while receiving from Kafka: {:?}", kafka_error);
                    None
                }
            }
        })
        .for_each(|msg| {
            // Process each message
            println!("Enqueuing message for computation, {:?}",
                     msg.payload_view::<str>());
            Ok(())
        });

    println!("Starting event loop");
    // Runs the event pool until the consumer terminates.
    core.run(processed_stream).unwrap();
    println!("Stream processing terminated");
}

fn main() {
    let matches = App::new("Kafka Consumer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Kafka consumer")
        .arg(Arg::with_name("brokers")
            .short("b")
            .long("brokers")
            .help("Broker list in kafka format")
            .takes_value(true)
            .default_value("localhost:9092"))
        .arg(Arg::with_name("group-id")
            .short("g")
            .long("group-id")
            .help("Consumer group id")
            .takes_value(true)
            .default_value("example_consumer_group_id"))
        .arg(Arg::with_name("topic")
            .long("topic")
            .short("t")
            .help("Input topic")
            .takes_value(true)
            .required(true))
        .get_matches();

    let brokers = matches.value_of("brokers").unwrap();
    let group_id = matches.value_of("group-id").unwrap();
    let input_topic = matches.value_of("topic").unwrap();

    println!("{}", brokers);
    utils::setup_logger(false, Some("consumer:"));
    run_consumer(brokers, group_id, input_topic);
}