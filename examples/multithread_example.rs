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
use std::{collections::HashMap, sync::Arc};
use std::thread;

use wurfl::{wurfl_download, Wurfl, WurflCacheProvider};

// This example application spawns 16 threads that share a single WURFL engine object and
// perform a device detection and a get_all_device_ids, plus device calls to get_det_device_id, get_static_cap and get_virtual_cap
fn main() {

        

      
      // download WURFL file from the ScientiaMobile Snapshot generator (replace the sample URL with your own specific customer URL to avoid error 402)
      let download_res = wurfl_download("https://data.scientiamobile.com/xxxxx/wurfl.zip", "test_files");
      match download_res {
          Ok(_) => println!("WURFL file downloaded successfully"),
          Err(_) => println!("WURFL file downloaded FAILED: keeping default version"),
      }

    let wurfl_path = "test_files/wurfl.zip";
    let engine = match Wurfl::new(wurfl_path, None, None,
                                  WurflCacheProvider::NoCache, None) {
        Ok(e) => e,
        Err(we) => panic!("{}", we.msg)
    };

    println!("Created engine version: {} ", engine.get_api_version());
    let safe_engine = Arc::new(engine);

    for i in 1..16 {
        // we'll pass a safe engine reference to each new thread
        let engine_for_thread = Arc::clone(&safe_engine);
        let _detection = {
            println!("Spawning thread: {}", i);
            let t = thread::spawn(move || {

                let dev_ids = engine_for_thread.get_all_device_ids();
                println!("Got {} WURFL IDs ", dev_ids.len());
                
                let ua = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.0.0 Mobile Safari/537.36";
                
                let mut headers = HashMap::new();
                headers.insert("User-Agent", ua);
                headers.insert("Sec-CH-UA", "\"Chromium\";v=\"94\", \"HuaweiBrowser\";v=\"99\", \";Not A Brand\";v=\"99\"");
                headers.insert("Sec-CH-UA-Full-Version", "99.123.456");
                headers.insert("Sec-CH-UA-Platform", "Android");
                headers.insert("Sec-CH-UA-Platform-Version", "12");
                headers.insert("Sec-CH-UA-Model", "Pixel 6");

                let dev = engine_for_thread.lookup_with_headers(headers);
                if dev.is_ok() {
                    let d = dev.unwrap();
                    println!("Got {} WURFL ID for detected device ", d.get_device_id());
                    let static_cap_res = d.get_static_cap("brand_name");
                    match static_cap_res {
                        Ok(cap) => println!("Device brand {}  ", cap.unwrap()),
                        Err(e) => println!("Error getting static capability: {}", e.to_string()),
                        
                    }

                    
                    let vcap_res = d.get_virtual_cap("is_smartphone");
                    match vcap_res {
                        Ok(vcap) => println!("Device is smartphone? {}  ", vcap.unwrap()),
                        Err(e) => println!("Error getting virtual capability: {}", e.to_string()),
                    }

                    
                    println!("---------------------------------------------------------------------");
                } else {
                    panic!("error detecting device {}", dev.err().unwrap().to_string());
                }
            });
            // wait for all threads to end
            t.join().unwrap();
        };
        println!("Ending Multi thread application - main thread ");
    }
}