// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GREETER_SAY_HELLO: ::grpcio::Method<super::service::HelloRequest, super::service::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/greeter.Greeter/SayHello",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct GreeterClient {
    client: ::grpcio::Client,
}

impl GreeterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GreeterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn say_hello_opt(&self, req: super::service::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello(&self, req: super::service::HelloRequest) -> ::grpcio::Result<super::service::HelloReply> {
        self.say_hello_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hello_async_opt(&self, req: super::service::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::service::HelloReply> {
        self.client.unary_call_async(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello_async(&self, req: super::service::HelloRequest) -> ::grpcio::ClientUnaryReceiver<super::service::HelloReply> {
        self.say_hello_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Greeter {
    fn say_hello(&self, ctx: ::grpcio::RpcContext, req: super::service::HelloRequest, sink: ::grpcio::UnarySink<super::service::HelloReply>);
}

pub fn create_greeter<S: Greeter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GREETER_SAY_HELLO, move |ctx, req, resp| {
        instance.say_hello(ctx, req, resp)
    });
    builder.build()
}
