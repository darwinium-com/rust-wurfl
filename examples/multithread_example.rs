use std::sync::Arc;
use std::thread;

use wurfl::{Wurfl, WurflCacheProvider};

fn main() {

    // This example application spawns 16 threads that share a single WURFL engine object and
    // perform a device detection and a get_all_device_ids, plus device calls to get_det_device_id, get_capability and get_virtual_capability

    let wurfl_path = "test_files/wurfl.zip";
    let engine = match Wurfl::new(wurfl_path, None, None,
                                  WurflCacheProvider::NoCache, None) {
        Ok(e) => e,
        Err(we) => panic!("{}", we.msg)
    };

    println!("Created engine version: {} ", engine.get_api_version());
    let safe_engine = Arc::new(engine);

    let _detection = {
        let t = thread::spawn(move || {
            let engine_ref = &safe_engine;
            for i in 1..16 {
                println!("Spawning thread: {}", i);
                let dev_ids = engine_ref.get_all_device_ids();
                println!("Got {} device IDs ", dev_ids.len());
                let ua = "Mozilla/5.0 (Linux; U; Android 8.1.0; in-id; CPH1901 Build/OPM1.171019.026) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/53.0.2785.134 Mobile Safari/537.36 OppoBrowser/15.5.1.10";
                let dev = engine_ref.lookup_useragent(ua);
                if dev.is_ok() {
                    let d = dev.unwrap();
                    println!("Got {} device ID for detected device ", d.get_device_id());
                    println!("Device brand {}  ", d.get_capability("brand_name").unwrap());
                    println!("Device is smartphone? {}  ", d.get_virtual_capability("is_smartphone").unwrap());
                    println!("---------------------------------------------------------------------");
                } else {
                    panic!("error detecting device {}", dev.err().unwrap().to_string());
                }
            };
        });
        // wait for all threads to end
        t.join().unwrap();
        println!("Ending Multi thread application - main thread ");
    };
}