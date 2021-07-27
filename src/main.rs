use std::{process, thread};
use paho_mqtt::{QOS_0};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use std::time::Duration;
use logwatcher::{LogWatcher, LogWatcherAction};

#[macro_use]
extern crate log;

static URL: &str = "tcp://127.0.0.1:1883";


fn main() -> std::io::Result<()> {
    info!("Starting program ...");

    let cli = paho_mqtt::Client::new(URL).unwrap_or_else(|err| {
        error!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new().clean_session(true)
        // .client_id(std::env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "logs-publisher".to_string()))
        .finalize();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts) {
        error!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/data.json");
    let path = Path::new("/Users/urvoy/Developpement/MOB_MDGate/logs/2021-07-27 13-49-47/").join("MDGate.log");

    let mut log_watcher = LogWatcher::register(path).unwrap();

    log_watcher.watch(&mut move |line: String| {
        let msg = paho_mqtt::Message::new("toto/tata", line, QOS_0);
        cli.publish(msg).unwrap();
        LogWatcherAction::None
    });

    Ok(())
}


