mod api;
mod configuration;
mod docs;
mod error;
mod startup;

use std::{env, io, net::TcpListener};

use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use bis_in_memory::{
    establish_connection,
    models::{NewBook, Store},
};
use configuration::get_configuration;
use docs::ApiDoc;
use startup::run;

// Name of the application
const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[actix_web::main]
async fn main() -> io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    // Setup logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conn = establish_connection();
    let conn = load_fake_data(conn);

    log::info!("Starting {} HTTP server at {}", APP_NAME, address);
    let listener = TcpListener::bind(address)?;
    run(listener, conn)?.await?;
    Ok(())
}

fn load_fake_data(conn: Store) -> Store {
    conn.create_book(NewBook {
        title: "Jack Burton".to_string(),
        author: "Also Jack Burton".to_string(),
        date_published: chrono::offset::Utc::now().naive_utc().date(),
    });

    conn
}
