use wurfl::*;

fn main() {
    println!("Starting WURFL wrapper usage sample!");
    let wurfl_path = "/usr/share/wurfl/wurfl.zip";
    let wurfl_res = Wurfl::new(wurfl_path, None, None,
                               WurflCacheProvider::LRU, Some("100000"));
    let engine = match wurfl_res {
        Ok(engine) => engine,
        Err(error) => panic!("Problem initializing wurfl: {:?}", error),
    };
    println!("WURFL API Version created: {}", engine.get_api_version());
    println!("Last load time:  {}", engine.get_last_load_time());
    println!("WURFL info:  {}", engine.get_info());

    let capability_names = engine.get_all_caps();
    // get an iterator from it and print first element
    println!("Printing capability name from Iterator");
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

    let ua = "Mozilla/5.0 (Linux; Android 6.0.1; Redmi 4A Build/MMB29M; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/60.0.3112.116 Mobile Safari/537.36";
    let device_res2 = engine.lookup_useragent(ua);
    let d2 = match device_res2 {
        Ok(d2) => {
            println!("Device 2 ID: {}", d2.get_device_id());
            println!("Device 2 is smartphone?: {}", d2.get_virtual_capability("is_smartphone").unwrap());
            d2
        }
        Err(_) => panic!("Lookup user agent failed")
    };
    println!("Device 2 ID - outside result closure: {}", d2.get_device_id());

    // Multiple lookup results added to vector
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

    // print some data
    let cap_names: &[&str] = &["brand_name", "model_name"];
    println!("----- Complete device names of last 5 detections -----");
    for device in &results {
        println!("{}", device.get_virtual_capability("complete_device_name").unwrap());
        match device.get_capabilities(cap_names) {
            Ok(caps_map) => {
                for (k, v) in caps_map {
                    println!("Cap from get_capabilities <{}:{}>", k, v)
                }
            }
            Err(_) => panic!("device get_capabilities failed!")
        };
    }

    // NOTE: why am I using a reference here?
    // Because using the vector itself would move its ownership to the iterator generated under the hood in the for loop,
    // thus making impossible to use it again in the second loop and making the compiler complain
    println!("----- Check if it's a tablet for the last 5 detections -----");
    for device in &results {
        println!("Is device a tablet? {}", device.get_capability("is_tablet").unwrap());
    }
}
