use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use wurfl::{Wurfl, WurflCacheProvider};

const DEFAULT_WURFL_FILE_PATH: &str = "/usr/share/wurfl/wurfl.zip";

#[tokio::main]
pub async fn main() {

    // This example is a variation of the "getting started" server example of hyper framework,
    // with the addition of a function that performs a device detection and returns a string
    // containing the WURFL device ID and its brand, model and marketing name (if available).

    // by default, this example will use the default WURFL file path for Linux machines,
    // but tou can set a custom WURFL file path by setting the WURFL_FILE_PATH environment variable.

    // retrieve WURFL file path. First from a custom location
    let key = "WURFL_FILE_PATH";
    let wurfl_path = match std::env::var(key) {
        Ok(p) => p,
        _ => {
            println!("env var WURFL_FILE_PATH not set, using default value for WURFL file path");
            DEFAULT_WURFL_FILE_PATH.to_owned()
        }
    };

    // First, create a Wurfl engine instance with cache provider
    let wurfl_res = Wurfl::new(&*wurfl_path, None, None,
                               WurflCacheProvider::LRU, Some("100000"));
    let engine = match wurfl_res {
        Ok(engine) => engine,
        Err(error) => panic!("Problem initializing wurfl: {:?}", error),
    };
    println!("WURFL API Version created: {}", engine.get_api_version());
    // now make it thread safe
    let safe_engine = Arc::new(engine);

    // A `Service` is needed for every connection, so this
    // creates one wrapping our `detect` function.
    let make_svc = make_service_fn(move |_conn| {
        let safe_engine_clone = Arc::clone(&safe_engine);
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                let response = detect(req, &safe_engine_clone);
                async { Ok::<_, Infallible>(Response::new(Body::from(response))) }
            }))
        }
    });

    // We'll bind the server to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_svc);
    match server.await {
        Err(_) => panic!("An error occurred while running WURFL hyper server example, shutting down"),
        _ => (),
    }
}

// Actual device detection: returns a string with wurfl_id and virtual capability complete_device_name
fn detect(_req: Request<Body>, safe_engine: &Arc<Wurfl>) -> String {
    let device = match safe_engine.lookup_with_headers(_req.headers()) {
        Ok(d) => d,
        Err(_) => panic!("Error during lookup")
    };
    let body = format!("Detected device: {} - {} ", device.get_device_id(),
                       device.get_virtual_capability("complete_device_name").unwrap());
    return body;
}