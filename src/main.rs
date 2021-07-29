use std::{process};
use paho_mqtt::QOS_0;
use std::path::Path;


use log::{error, LevelFilter};
use logwatcher::{LogWatcher, LogWatcherAction};
use simple_logger::SimpleLogger;

use clap::{App, Arg};

#[macro_use]
extern crate log;

fn main() -> std::io::Result<()> {
    // create cli app and get Arg
    let matches = App::new("logs-publisher")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Urvoy L")
        .about("Read the log file and publish log to MQTT topic")
        .arg(
            Arg::with_name("file").short("f").long("file").help("Set the file to read").required(true).takes_value(true),
        )
        .arg(
            Arg::with_name("broker_url").short("u").long("broker_url").help("Set the URL of MQTT broker").required(true).takes_value(true)
        )
        .arg(
            Arg::with_name("topic").short("t").long("topic").help("MQTT Topic target for publish").required(true).takes_value(true)
        )
        .get_matches();

    let broker_url = matches.value_of("broker_url").unwrap();
    let file_path = matches.value_of("file").unwrap();
    let topic = matches.value_of("topic").unwrap();

    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    info!("Starting program ...");

    let cli = paho_mqtt::Client::new(broker_url).unwrap_or_else(|err| {
        error!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new().clean_session(true)
        .finalize();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts) {
        error!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    info!("MQTT Connected to {}", broker_url);

    let path = Path::new(file_path);

    let mut log_watcher = LogWatcher::register(path).unwrap();

    log_watcher.watch(&mut move |line: String| {
        let msg = paho_mqtt::Message::new(topic, line.clone(), QOS_0);
        if cli.is_connected() {
            cli.publish(msg).unwrap();
            info!("{}", line);
        } else {
            info!("MQTT cli not connected when publish, try to reconnect to {}", broker_url);
            if cli.reconnect().is_ok() {
                cli.publish(msg).unwrap();
                info!("{}", line);
            } else {
                warn!("Fail to MQTT reconnect");
            }
        }
        LogWatcherAction::None
    });

    Ok(())
}
