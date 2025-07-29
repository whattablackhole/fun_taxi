use crate::{
    api::gps_service_messaging::outcoming_messages::{
        DriverLocationChangedMessage, GeoLocation,
        GpsMessageType, DriverMessage,
    },
    gps::gps::GPS,
    models::geoposition::GeoPosition,
    shared::utils::{to_degrees, to_radians},
};
use chrono::DateTime;
use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message, Utf8Bytes},
};
use tokio_util::sync::CancellationToken;

pub struct DriverBot {
    id: String,
    state_receiver: Option<tokio::sync::mpsc::Receiver<String>>,
    cmd_sender: tokio::sync::watch::Sender<Option<tokio::sync::mpsc::Sender<String>>>,
    gps_service_sender:
        tokio::sync::watch::Sender<Option<tokio::sync::mpsc::Sender<GpsServiceMessage>>>,
    pub connection_handle: Option<JoinHandle<()>>,
    pub car_handle: Option<JoinHandle<()>>,
    driver_service_url: String,
    gps_service_url: String,
}

#[derive(Debug)]
pub enum CarMessage {
    Update(GeoPosition),
}

#[derive(Debug)]
pub enum GpsServiceMessage {
    Update(GeoPosition),
}

pub struct Car {
    gps: Arc<RwLock<GPS>>,
    state_sender: tokio::sync::mpsc::Sender<CarMessage>,
    rx_command: tokio::sync::mpsc::Receiver<String>,
}

impl Car {
    pub fn new(initial_position: GeoPosition) -> (Self, Sender<String>, Receiver<CarMessage>) {
        let (state_tx, state_rx) = tokio::sync::mpsc::channel(32);
        let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(32);

        let car = Car {
            gps: Arc::new(RwLock::new(GPS::new(initial_position))),
            state_sender: state_tx,
            rx_command: cmd_rx,
        };

        (car, cmd_tx, state_rx)
    }

    pub async fn listen(&mut self) {
        let token: CancellationToken = CancellationToken::new();

        while let Some(cmd) = self.rx_command.recv().await {
            if cmd == "Drive" {
                self.drive(GeoPosition::new(8.687872, 49.420318), &token)
                    .await;
            } else if cmd == "Stop" {
                token.cancel();
            }
        }
    }

    pub async fn drive(&mut self, destination: GeoPosition, cancelation_token: &CancellationToken) {
        {
            self.gps.write().await.load_route(destination).await;
        }

        while let Some(next_pos) = tokio::select! {
            _ = cancelation_token.cancelled() => None,
            pos = async {
                let mut gps_handle = self.gps.write().await;
                gps_handle.get_next_position()
            } => pos,
        } {
            let new_current_pos = {
                let gps = self.gps.read().await;

                let curr_pos = gps.get_current_position();
                let new_current_pos = self.move_car(curr_pos, &next_pos, 40.0);
                new_current_pos
            };

            {
                // let payload: String = serde_json::to_string(&new_current_pos).unwrap();

                self.gps.write().await.update_curr_pos(&new_current_pos);
                self.state_sender
                    .send(CarMessage::Update(new_current_pos))
                    .await
                    .unwrap();
            }

            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }

    fn move_car(&self, curr_pos: &GeoPosition, next_pos: &GeoPosition, speed: f64) -> GeoPosition {
        const R: f64 = 6371000.0;
        let bearing = to_radians(self.calculate_bearing(curr_pos, next_pos));

        let cur_lat = to_radians(curr_pos.lat);
        let cur_lon = to_radians(curr_pos.lon);

        let distance = speed / R;

        let new_lat = (cur_lat.sin() * distance.cos()
            + cur_lat.cos() * distance.sin() * bearing.cos())
        .asin();

        let new_lon = cur_lon
            + (bearing.sin() * distance.sin() * cur_lat.cos())
                .atan2(distance.cos() - cur_lat.sin() * new_lat.sin());

        GeoPosition::new(to_degrees(new_lat), to_degrees(new_lon))
    }

    fn calculate_bearing(&self, curr_pos: &GeoPosition, next_pos: &GeoPosition) -> f64 {
        let delta_lon = to_radians(next_pos.lon - curr_pos.lon);
        let current_lat = to_radians(curr_pos.lat);
        let next_lat = to_radians(next_pos.lat);

        let x = delta_lon.sin() * next_lat.cos();
        let y = current_lat.cos() * next_lat.sin()
            - current_lat.sin() * next_lat.cos() * delta_lon.cos();

        let bearing = f64::atan2(x, y);
        (to_degrees(bearing) + 360.0) % 360.0
    }
}

impl DriverBot {
    pub fn new(id: String) -> DriverBot {
        return Self {
            id: id,
            state_receiver: None,
            cmd_sender: tokio::sync::watch::Sender::new(None),
            gps_service_sender: tokio::sync::watch::Sender::new(None),
            connection_handle: None,
            car_handle: None,
            driver_service_url: std::env::var("DRIVER_SERVICE_WS_URL").unwrap(),
            gps_service_url: std::env::var("GPS_SERVICE_WS_URL").unwrap(),
        };
    }

    pub async fn start_car(
        &mut self,
        mut car: Car,
        mut state_receiver: Receiver<CarMessage>,
        cmd_sender: Sender<String>,
    ) {
        let car_handle = tokio::spawn(async move {
            car.listen().await;
        });

        self.car_handle = Some(car_handle);
        self.cmd_sender.send_replace(Some(cmd_sender));

        let mut receiver: tokio::sync::watch::Receiver<Option<Sender<GpsServiceMessage>>> =
            self.gps_service_sender.subscribe();

        tokio::spawn(async move {
            loop {
                if let Some(msg) = state_receiver.recv().await {
                    receiver.wait_for(|v| v.is_some()).await.unwrap();
                    let option = receiver.borrow().as_ref().cloned();
                    if let Some(sender) = option {
                        match msg {
                            CarMessage::Update(position) => {
                                sender
                                    .send(GpsServiceMessage::Update(position))
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        });
    }

    pub async fn establish_connection(&mut self) {
        let id = self.id.clone();
        let request = self
            .driver_service_url
            .clone()
            .into_client_request()
            .unwrap();
        let mut connection = connect_async(request).await.unwrap().0;
        let request = self.gps_service_url.clone().into_client_request().unwrap();
        let mut gps_connection = connect_async(request).await.unwrap().0;
        let handshake = serde_json::json!({
            "protocol": "json",
            "version": 1
        })
        .to_string()
            + "\u{1e}";
        gps_connection
            .send(Message::Text(handshake.into()))
            .await
            .unwrap();

            gps_connection
            .send(Message::Text(self.signal_r_message().into()))
            .await
            .unwrap();  
        let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel::<GpsServiceMessage>(32);
        self.gps_service_sender.send_replace(Some(cmd_tx));

        // TODO: use seperate channel for handling messages betwee Connection <-> Driver <-> Car
        // currently it use such pattern
        // Driver -> Connection
        // Connection -> Car
        let receiver = self.cmd_sender.subscribe();
        let connection_handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = connection.next() => {
                         match msg {
                            Ok(e) => {
                                let sender_option = receiver.borrow().clone();
                                if let Some(sender) = sender_option {
                                     let text_message = e.to_text().unwrap();
                                        if text_message == "Drive" {
                                            sender.send(String::from("Drive")).await.unwrap();
                                        }
                                }

                            },
                            Err(e) => {}

                        }
                    }
                    Some(cmd) = cmd_rx.recv() => {
                        match cmd {
                            GpsServiceMessage::Update(geo_position) => {
                                gps_connection.send(Message::Text(Utf8Bytes::from_static("w"))).await.unwrap();
                            },
                        }
                    }
                }
            }
        });

        self.connection_handle = Some(connection_handle);
    }

    fn signal_r_message(&self) -> String {
        // test
        let msg = DriverMessage {
            r#type: GpsMessageType::LocationUpdate,
            driver_id: "1".to_string(),
            payload: DriverLocationChangedMessage {
                location: GeoLocation { lat: 5.0, lon: 5.0 },
                sent_at: DateTime::default(),
            },
        };
        let signalr_message = format!(
            "{}\u{001e}",
            serde_json::json!({
                "type": 1,
                "target": "SendMessage",
                "arguments": [msg]
            })
        );
        return signalr_message;
    }
}
