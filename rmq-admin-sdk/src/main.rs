mod modules;

use modules::client::Rabbitmq;

#[tokio::main]
async fn main() {
    let rabbitmq = Rabbitmq::new(
        "http://localhost".to_string(),
        5672,
        "guest".to_string(),
        "guest".to_string(),
        10,
    );
}
