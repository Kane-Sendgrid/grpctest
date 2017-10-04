# Rust GRPC and Kafka

Command line kafka consumer and producer

## Build docker images and start kafka
```bash
docker-compose build service
docker-compose up -d kafka service
```

## Exec into rust container
```bash
docker-compose exec service bash
```

## Run producer and consumer (in different exec sessions)
```bash
cargo run --bin producer -- -t transactions -b kafka:9092
cargo run --bin consumer -- -t transactions -b kafka:9092
```