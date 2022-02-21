use std::{collections::HashMap, io};

use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

mod user;
use time::{format_description, macros::offset};
use tokio::sync::Mutex;
use tracing::{debug, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    filter,
    fmt::{self, time::OffsetTime},
    prelude::__tracing_subscriber_SubscriberExt,
};
use user::{api::Api, Token};
const FORMAT_STR: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _guard = init_log(true);

    tokio::spawn(refresh_token());

    let api_service =
        OpenApiService::new(Api::default(), "Users", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/", ui)
                .with(AddData::new(Token("token123".to_string()))),
        )
        .await
}

fn init_log(verbose: bool) -> WorkerGuard {
    let (non_blocking, _guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily("log", "info.log"));

    let timer = OffsetTime::new(
        offset!(+8),
        format_description::parse(FORMAT_STR).expect("parse format error"),
    );

    let subscriber = tracing_subscriber::registry()
        .with(filter::Targets::new().with_target(
            "poem_demo",
            if verbose { Level::DEBUG } else { Level::INFO },
        ))
        .with(
            fmt::Layer::new()
                .with_timer(timer.clone())
                .with_writer(io::stdout), // .with_filter(LevelFilter::TRACE),
        )
        .with(
            fmt::Layer::new()
                .with_timer(timer)
                .with_ansi(false)
                .with_writer(non_blocking), // .with_filter(LevelFilter::TRACE),
        );

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");

    _guard
}

async fn refresh_token() {
    let mut interval = 0;

    let mut count = 0;
    loop {
        count = count + 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
        debug!("refresh_token... ");

        interval = 5;
        CACHE
            .lock()
            .await
            .insert("token".to_string(), format!("token{}", count));
    }
}
