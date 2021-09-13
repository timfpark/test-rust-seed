use axum::{handler::get, Router};
use lazy_static::lazy_static;
use prometheus::{IntCounter, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref HITS: IntCounter =
        IntCounter::new("hits", "Number of hits").expect("hits could not be created");
}

#[tokio::main]
async fn main() {
    REGISTRY
        .register(Box::new(HITS.clone()))
        .expect("hits could not be registered");

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                HITS.inc();
                "Hello, World!"
            }),
        )
        .route("/metrics", get(metrics_handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn metrics_handler() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    }
    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    }
    let res_prom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_prom);
    res
}
