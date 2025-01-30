use std::time::Duration;

use paho_mqtt::{Client, ConnectOptionsBuilder, MessageBuilder};

use crate::mqtt_settings::MqttSettings;

pub struct MqttClient {
    client: Option<Client>,
    pub settings: Option<MqttSettings>,
}

impl MqttClient {
    pub fn new() -> Self {
        Self {
            client: None,
            settings: None,
        }
    }
    pub fn connect(&mut self) {
        if let Some(client) = &self.client {
            let _ = client.disconnect(None);
        }
        self.client = None;
        let client = Client::new(format!(
            "tcp://{}:{}",
            self.settings.as_ref().unwrap().ip,
            self.settings.as_ref().unwrap().port
        ));

        match client {
            Ok(mut client) => {
                client.set_timeout(Duration::from_secs(5));
                let interval = Duration::new(1, 0);
                match client.connect(
                    ConnectOptionsBuilder::new()
                        .automatic_reconnect(interval, interval)
                        .finalize(),
                ) {
                    Ok(_) => self.client = Some(client),
                    Err(_) => self.client = None,
                };
            }
            Err(_) => self.client = None,
        };
    }

    pub fn publish(&self, topic: &String, value: &String) {
        if let Some(client) = &self.client {
            let msg = MessageBuilder::new()
                .topic(topic)
                .payload(value.clone())
                .qos(3)
                .retained(true)
                .finalize();

            if let Err(e) = client.publish(msg) {
                println!("Error publishing message: {}", e);
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        if let Some(client) = &self.client {
            return client.is_connected();
        }
        false
    }
}
