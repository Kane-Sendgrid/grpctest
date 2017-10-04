extern crate futures;
extern crate grpcio;

use proto::service;
use proto::service_grpc;
use std::sync::Arc;
use std::io::Read;
use std::{io, thread};
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use futures::Future;
use futures::sync::oneshot;

#[derive(Clone)]
struct GreeterService;

impl service_grpc::Greeter for GreeterService {
    fn say_hello(&self,
                 ctx: ::grpcio::RpcContext,
                 req: service::HelloRequest,
                 sink: ::grpcio::UnarySink<service::HelloReply>) {
        println!(">>>> got request");
        let msg = format!("Hello {}", req.get_name());
        let mut resp = service::HelloReply::new();
        resp.set_message(msg);
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

pub fn start() {
    let env = Arc::new(Environment::new(1));
    let service = service_grpc::create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}