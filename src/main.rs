mod wifi;
use embedded_svc::ws::Sender;
use esp_idf_svc::ws::client::{EspWebSocketClient, EspWebSocketClientConfig, WebSocketEventType};
use esp_idf_sys as _;
use std::time::Duration;
use wifi::wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_passwd: &'static str,
    #[default("")]
    ws_uri: &'static str,
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    let _w = wifi(CONFIG.wifi_ssid, CONFIG.wifi_passwd)?;
    let mut ws_client = EspWebSocketClient::new(
        CONFIG.ws_uri,
        &EspWebSocketClientConfig::default(),
        Duration::from_millis(60000),
        |ev| {
            if let Ok(ws_event) = ev {
                match ws_event.event_type {
                    WebSocketEventType::Connected => {
                        println!("Connected to server");
                    }
                    WebSocketEventType::Disconnected => {
                        println!("Disconnected from server");
                    }
                    WebSocketEventType::Close(close) => {
                        if let Some(reason) = close {
                            println!("Disconnected from server: {:#?}", reason);
                        }
                    }
                    WebSocketEventType::Closed => {
                        println!("Connection closed");
                    }
                    WebSocketEventType::Binary(data) => {
                        println!("Binary data Received: {:#?}", data);
                    }
                    WebSocketEventType::Text(data) => {
                        println!("Text data Received: {:#?}", data);
                    }
                }
            }
        },
    )?;

    for i in 1..=4 {
        ws_client.send(
            embedded_svc::ws::FrameType::Text(false),
            Some(format!("Hello Server {}", i).as_bytes()),
        )?;
    }

    Ok(())
}
