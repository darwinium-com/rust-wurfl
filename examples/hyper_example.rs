/*
  WURFL API for Rust - Commercial Use License
  This software module is a proprietary component of ScientiaMobile's WURFL API for Rust. Access and use of this code are strictly governed by the terms of a valid commercial license agreement with ScientiaMobile, Inc.
  License Terms:

  Commercial License Required: This module can only be used by entities or individuals who have purchased and maintain an active commercial license from ScientiaMobile, Inc.
  Prohibited Use: Any use, modification, distribution, or reproduction of this module without an explicit commercial license from ScientiaMobile, Inc. is strictly prohibited.
  Compliance: Users must comply with all terms, conditions, and restrictions set forth in their specific commercial license agreement with ScientiaMobile, Inc.
  No Open Source: This module is not open source, and no rights are granted under any open source license terms.
  Enforcement: ScientiaMobile, Inc. reserves the right to enforce compliance with this license, including but not limited to, legal action against unauthorized use.

  For License Information:

  Please contact ScientiaMobile, Inc. at  https://www.scientiamobile.com/general-inquiry/

  IMPORTANT: This notice must remain intact in all copies or substantial portions of the Software.
*/
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use wurfl::{wurfl_download, Wurfl, WurflCacheProvider};

// We use WURFL file included in the libwurfl installation (in case of Windows OS installations, change this default path)
const DEFAULT_WURFL_FILE_PATH: &str = "/usr/share/wurfl/wurfl.zip";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // This example is a variation of the "getting started" server example of hyper framework,
    // with the addition of a function that performs a device detection and returns a string
    // containing the device's WURFL ID and its brand, model and marketing name (if available).

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

    // Before we create a Wurfl instance, we download an updated WURFL file from the ScientiaMobile Snapshot generator 
    // (replace the sample URL with your own specific customer URL to avoid error 402)
    let download_res = wurfl_download("https://data.scientiamobile.com/xxxxx/wurfl.zip", ".");
    match download_res {
        Ok(_) => println!("WURFL file downloaded successfully"),
        Err(_) => println!("WURFL file downloaded FAILED: keeping default version"),
    }

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

    // We'll bind the server to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let safe_engine_clone = Arc::clone(&safe_engine);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| {
                    let safe_engine_clone = Arc::clone(&safe_engine_clone);
                    async move {
                        let response = detect(req, &safe_engine_clone);
                        Ok::<_, Infallible>(Response::new(response))
                    }
                }))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

// Actual device detection: returns a string with wurfl_id and virtual capability complete_device_name
fn detect(req: Request<hyper::body::Incoming>, safe_engine: &Arc<Wurfl>) -> String {
    let device = match safe_engine.lookup_with_headers(req.headers()) {
        Ok(d) => d,
        Err(_) => panic!("Error during lookup")
    };

    let compl_device_name_res = device.get_virtual_cap("complete_device_name");
    let complete_device_name = match compl_device_name_res {
        Ok(v) => v.unwrap(),
        Err(_) => "unknown"
        
    };

    let body = format!("Detected device: {} - {} ", device.get_device_id(), complete_device_name);
    return body;
}