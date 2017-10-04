#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate futures_await as futures;

#[macro_use]
extern crate log;
extern crate clap;
extern crate rdkafka;
extern crate tokio_core;

use futures::prelude::*;
use futures::sync::mpsc::{channel, Receiver};

mod utils;

use rdkafka::producer::FutureProducer;
use futures::stream::Stream;
use std::{io, thread};
use std::io::Write;

use clap::{App, Arg};
use tokio_core::reactor::Core;
use rdkafka::client::Context;
use rdkafka::config::{ClientConfig, TopicConfig};
use rdkafka::error::KafkaError;
// use rdkafka::message::Message;

pub struct LocalContext;
impl Context for LocalContext {
    fn error(&self, error: KafkaError, reason: &str) {
        error!(">>>>>>>>>>>>>>>>> librdkafka: {}: {}", error, reason);
    }
}

fn run_producer(recv: Receiver<String>, brokers: &str, topic: &str) {
    // Create the event loop. The event loop will run on a single thread and drive the pipeline.
    let mut core = Core::new().unwrap();
    let producer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set_default_topic_config(TopicConfig::new()
            .set("produce.offset.report", "true")
            .finalize())
        .create::<FutureProducer<_>>()
        .expect("Producer creation error");
    println!("Starting event loop");

    let server = async_block! {
        #[async]
        for item in recv {
            println!(">>>>>>>>>>>> got in server: {:?}", item);
            let result = await!(producer.send_copy::<String, ()>(&topic, None, Some(&item), None, None));
            println!("{:?}", result);
        }
        Ok::<(), ()>(())
    };

    // Runs the event pool until the consumer terminates.
    core.run(server).unwrap();
    println!("Stream processing terminated");
}

fn main() {
    let matches = App::new("Kafka Producer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Kafka consumer")
        .arg(Arg::with_name("brokers")
            .short("b")
            .long("brokers")
            .help("Broker list in kafka format")
            .takes_value(true)
            .default_value("localhost:9092"))
        .arg(Arg::with_name("topic")
            .long("topic")
            .short("t")
            .help("Output topic")
            .takes_value(true)
            .required(true))
        .get_matches();

    utils::setup_logger(false, Some("producer:"));

    let (mut send, recv) = channel::<String>(1);
    thread::spawn(move || {
        let brokers = matches.value_of("brokers").unwrap();
        let topic = matches.value_of("topic").unwrap();
        run_producer(recv, brokers, topic);
    });    

    loop {
        let mut input = String::new();
        io::stdout().flush().ok().expect("Could not flush stdout");
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                send.try_send(input);
            }
            Err(error) => println!("error: {}", error),
        }    
    }
}