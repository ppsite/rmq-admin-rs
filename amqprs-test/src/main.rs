use amqprs::{
    channel::{BasicPublishArguments, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use clap::{Parser, ValueEnum};
use log::info;
use tokio::time;

#[derive(Parser, Debug, Clone, ValueEnum)]
pub enum Action {
    Publish,
    Consume,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // action
    #[arg(value_enum, short, long, default_value_t = Action::Publish)]
    action: Action,
}

// define structs with fields: host, port, username, password, queue, routing_key, exchange_name
pub struct Rabbitmq {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub queue: String,
    pub routing_key: String,
    pub exchange_name: String,
}

impl Rabbitmq {
    fn default() -> Self {
        Rabbitmq {
            host: "localhost".to_string(),
            port: 5672,
            username: "guest".to_string(),
            password: "guest".to_string(),
            queue: "amqprs.test".to_string(),
            routing_key: "amqprs.test".to_string(),
            exchange_name: "amq.topic".to_string(),
        }
    }

    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
        queue: String,
        routing_key: String,
        exchange_name: String,
    ) -> Self {
        let mut rabbitmq = Rabbitmq::default();
        rabbitmq.host = host;
        rabbitmq.port = port;
        rabbitmq.username = username;
        rabbitmq.password = password;
        rabbitmq.queue = queue;
        rabbitmq.routing_key = routing_key;
        rabbitmq.exchange_name = exchange_name;
        rabbitmq
    }

    pub async fn publish(&self) {
        info!("Start publishing message ...");
        // create connection
        // open a connection to RabbitMQ server
        let connection = Connection::open(&OpenConnectionArguments::new(
            self.host.as_str(),
            5672,
            self.username.as_str(),
            self.password.as_str(),
        ))
        .await
        .unwrap();
        // create channel
        let channel = connection.open_channel(None).await.unwrap();
        // declare a queue
        let (queue_name, _, _) = channel
            .queue_declare(QueueDeclareArguments::new(self.queue.as_str()))
            .await
            .unwrap()
            .unwrap();
        // bind the queue to exchange
        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                self.exchange_name.as_str(),
                self.routing_key.as_str(),
            ))
            .await
            .unwrap();

        // publish message
        // create arguments for basic_publish
        let args =
            BasicPublishArguments::new(self.exchange_name.as_str(), self.routing_key.as_str());

        for i in 0..1000 {
            // create content
            let content = format!("hello world {}", i);
            info!("Published message: {}", content);
            channel
                .basic_publish(
                    BasicProperties::default(),
                    content.into_bytes(),
                    args.clone(),
                )
                .await
                .unwrap();
            time::sleep(time::Duration::from_secs(1)).await;
        }
    }

    pub fn consume(&self) {
        info!("Start consuming message ...");
    }
}

#[tokio::main]
async fn main() {
    // init logger
    env_logger::init();

    // parse the arguments
    let args = Args::parse();

    // rabbitmq instance
    let rabbitmq = Rabbitmq::default();
    match args.action {
        Action::Publish => rabbitmq.publish().await,
        Action::Consume => rabbitmq.consume(),
    }
}
