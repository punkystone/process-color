use std::time::Duration;

use paho_mqtt::{Client, ConnectOptionsBuilder, MessageBuilder};

use crate::{log, mqtt_settings::MqttSettings};

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
            let result = client.disconnect(None);
            if result.is_err() {
                log("error disconnecting from MQTT broker");
            }
        }
        self.client = None;
        if self.settings.is_none() {
            return;
        }
        let settings = self.settings.as_ref().unwrap();
        let client = Client::new(format!("tcp://{}:{}", settings.ip, settings.port));
        if client.is_err() {
            self.client = None;
            log("error creating MQTT client");
            return;
        }
        let mut client = client.unwrap();
        client.set_timeout(Duration::from_secs(5));
        let interval = Duration::new(1, 0);
        let response = client.connect(
            ConnectOptionsBuilder::new()
                .automatic_reconnect(interval, interval)
                .finalize(),
        );
        if response.is_err() {
            self.client = None;
            log("error connecting to MQTT broker");
            return;
        }
        self.client = Some(client);
    }

    pub fn publish(&self, topic: &String, value: &String) {
        if let Some(client) = &self.client {
            let msg = MessageBuilder::new()
                .topic(topic)
                .payload(value.clone())
                .qos(3)
                .retained(true)
                .finalize();

            if client.publish(msg).is_err() {
                log("error publishing message");
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
