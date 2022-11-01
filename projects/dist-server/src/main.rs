use std::net::{IpAddr, Ipv6Addr, SocketAddr};


use axum::{response::IntoResponse, Router, routing::get};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "dist", about = "A dist-server for our wasm project!")]
pub struct Args {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}
use local_ip_address::local_ip;


#[tokio::main]
async fn main() {
    let opt = Args::parse();
    let my_local_ip = local_ip().unwrap();
    println!("This is my local IP address: {:?}", my_local_ip);
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/api/hello", get(hello))
        .merge(SpaRouter::new("/assets", opt.static_dir))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));
    let sock_addr = SocketAddr::from((IpAddr::V6(Ipv6Addr::LOCALHOST), opt.port, ));
    log::info!("listening on http://{}", sock_addr);
    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start dist-server");
}

async fn hello() -> impl IntoResponse {
    "hello from dist-server!"
}
