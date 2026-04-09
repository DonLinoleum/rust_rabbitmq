use lapin::{Channel, Connection, ConnectionProperties, options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions}, types::{FieldTable}};
use futures_util::StreamExt;

use crate::{models::VisitRequest, visits::{visits_repository}};

pub async fn create_channel(amqp_url: &str) -> (Connection, Channel)
{
    let conn = Connection::connect(amqp_url, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ!!!");
    let channel = conn.create_channel().await.expect("Failed to create RabbitMQ channel!!!");
    (conn, channel)
}

pub async fn subscribe_to_visits(channel: Channel, pool: sqlx::PgPool){
    let _queue = channel.queue_declare("score_logs".into(), QueueDeclareOptions{durable:true, ..Default::default()}, FieldTable::default())
        .await
        .expect("Failed to declare RabbitMQ queue!!!");
    let mut consumer = channel.basic_consume("score_logs".into(), "".into(), BasicConsumeOptions::default(), FieldTable::default())
        .await
        .expect("Failed to consume RabbitMQ consumer!!!");

    while let Some(delivery) = consumer.next().await{
        match delivery{
            Ok(delivery) => {
                match serde_json::from_slice::<VisitRequest>(&delivery.data){
                    Ok(visit) => {
                            if let Err(e) = visits_repository::insert_visit(&pool, visit).await{
                                eprintln!("Failed to insert visit from RabbitMQ message: {:?}", e);
                            }
                            delivery.ack(BasicAckOptions::default()).await.ok();
                    }
                    Err(e) => {
                            eprintln!("Skipping bad message: {}. Error: {}", String::from_utf8_lossy(&delivery.data), e);
                            delivery.ack(BasicAckOptions::default()).await.ok();
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to receive RabbitMQ message: {:?}", e);
            }
        }
    }
}