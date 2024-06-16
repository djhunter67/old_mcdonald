mod identifier;
mod weather;

use actix_web::{http::KeepAlive, web, App, HttpServer};
use log::{debug, info, warn, LevelFilter};

use redis::Client;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::{fs::File, process::exit};

use weather::gov_api::get_weather;

const HOST_IP: &str = "0.0.0.0"; // Local connection of this server
const PORT: u16 = 8086; // PORT with which to access this server
                        // const IP_VAR: &str = "192.168.3.99:7531"; // USB connection to Matchstick S-10
/// The path to the redis database
pub const REDIS_PATH: &str = "redis://127.0.0.1/"; // Local connection on dev machine

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // This is a macro that allows for multiple loggers to be used at once
    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Always,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("old_mcdonald.log")?,
        ),
    ]) {
        Ok(()) => debug!("Logger initialized."),
        Err(e) => {
            log::error!("Error initializing logger: {e:?}");
            exit(1);
        }
    }

    info!("Running on port: {PORT}");

    let redis_client = Client::open(REDIS_PATH).unwrap();
    let client = reqwest::Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .service(get_weather)
    })
    .keep_alive(KeepAlive::Os) // Keep the connection alive; OS handled
    .bind((HOST_IP, PORT))
    .unwrap_or_else(|_| {
        warn!("Error binding to port {}.", PORT);
        std::process::exit(1); // This is expected behavior if the port is already in use
    })
    .disable_signals() // Disable the signals to allow the OS to handle the signals
    .shutdown_timeout(3)
    .workers(2)
    .run()
    .await
}
