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
use std::collections::HashMap;

use wurfl::*;

fn main() {
    println!("Starting WURFL wrapper usage sample!");

    // example assumes that the WURFL file is located in the current directory
    let wurfl_path = "wurfl.zip";

    // download WURFL file from the ScientiaMobile Snapshot generator (replace the sample URL with your own specific customer URL to avoid error 402)
    let download_res = wurfl_download("https://data.scientiamobile.com/xxxxx/wurfl.zip", ".");
    match download_res {
        Ok(_) => println!("WURFL file downloaded successfully"),
        Err(_) => println!("WURFL file downloaded FAILED: keeping default version"),
    }

    let wurfl_res = Wurfl::new(wurfl_path, None, None,
                               WurflCacheProvider::LRU, Some("100000"));
    let engine = match wurfl_res {
        Ok(engine) => engine,
        Err(error) => panic!("Problem initializing wurfl: {:?}", error),
    };
    println!("WURFL API Version created: {}", engine.get_api_version());
    println!("Last load time:  {}", engine.last_load_time());
    println!("WURFL info:  {}", engine.info());

    let capability_names = engine.get_all_caps();
    // get an iterator from it and print first element
    println!("Printing capability names from Iterator");
    let mut i = capability_names.iter();
    let s = i.next();
    println!("> {} <", s.unwrap());
    // print all capabilities name via for loop (= implicit IntoIterator usage)
    println!("Printing all capability names --------------------------");
    for c in capability_names {
        println!("< {} >", c);
    }
    println!("---------------------------------------------------------");

    println!("Printing all virtual capability names -------------------");
    let vcap_names = engine.get_all_vcaps();
    for c in vcap_names {
        println!("< {} >", c);
    }
    println!("---------------------------------------------------------");

    let ua = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.0.0 Mobile Safari/537.36";
    
    let mut headers = HashMap::new();
    headers.insert("User-Agent", ua);
    headers.insert("Sec-CH-UA", "\"Chromium\";v=\"94\", \"HuaweiBrowser\";v=\"99\", \";Not A Brand\";v=\"99\"");
    headers.insert("Sec-CH-UA-Full-Version", "99.123.456");
    headers.insert("Sec-CH-UA-Platform", "Android");
    headers.insert("Sec-CH-UA-Platform-Version", "12");
    headers.insert("Sec-CH-UA-Model", "Pixel 6");


    let device_res2 = engine.lookup_with_headers(headers);
    let d2 = match device_res2 {
        Ok(d2) => {
            println!("Device 2 ID: {}", d2.get_device_id());
            let is_smartphone_res = d2.get_virtual_cap("is_smartphone");
            match is_smartphone_res {
                Ok(is_smartphone) => {
                    println!("Device 2 is smartphone?: {}", is_smartphone.unwrap());
                }
                Err(_) => panic!("Getting virtual capability failed")
            }
            d2
        }
        Err(_) => panic!("Lookup with headers failed")
    };
    println!("Device 2 ID - outside result closure: {}", d2.get_device_id());

    // Multiple lookup using user-agent only: we add the results to a vector
    let uas: &[&str] = &[
        "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 13_3_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.5 Mobile/15E148 Snapchat/10.77.5.59 (like Safari/604.1)",
        "Mozilla/5.0 (X11; Linux x86_64; rv:45.0) Gecko/20100101 Thunderbird/45.3.0",
        "Mozilla/5.0 (BlackBerry; U; BlackBerry 9780; en) AppleWebKit/534.8+ (KHTML, like Gecko) Version/6.0.0.480 Mobile Safari/534.8+",
        "Mozilla/5.0 (Linux; U; Android 5.0; en-US; E2303 Build/26.1.A.3.111) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.108 UCBrowser/12.9.10.1159 Mobile Safari/537.36"
    ];
    let mut results = Vec::new();
    for ua in uas {
        let rd = engine.lookup_useragent(ua);
        match rd {
            Ok(d) => results.push(d),
            Err(_) => panic!("Lookup user agent failed")
        };
    }

    // print some data or panic is some error occurs
    let cap_names: &[&str] = &["brand_name", "model_name"];
    println!("----- Complete device names of last 5 detections -----");
    for device in &results {
        let vcap_res = device.get_virtual_cap("complete_device_name");
match vcap_res {
    Ok(vcap) => println!("Complete device name: {}", vcap.unwrap()),
    Err(_) => panic!("get_virtual_cap failed")
    
}

        match device.get_static_capabilities(cap_names) {
            Ok(caps_map) => {
                for (k, v) in caps_map {
                    println!("Cap from get_capabilities <{}:{}>", k, v)
                }
            }
            Err(_) => panic!("device get_capabilities failed!")
        };
    }

    // NOTE: why am I using a reference to results here?
    // Because using the vector itself would move its ownership to the iterator generated under the hood in the for loop,
    // thus making impossible to use it again in the second loop and making the compiler complain
    println!("----- Check if it's a tablet for the last 5 detections -----");
    for device in &results {
        let is_tablet_res = device.get_static_cap("is_tablet");
        match is_tablet_res {
            Ok(is_tablet) => {
                println!("Is device a tablet? {}", is_tablet.unwrap());
            }
            Err(_) => panic!("get_static_capability failed")
        }
    }
}
