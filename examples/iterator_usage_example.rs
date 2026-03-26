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
use wurfl::{Wurfl, WurflCacheProvider};
use wurfl::wurfl_download;

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

    let dev_id = "google_pixel_8_ver1";
    let device = match engine.lookup_device_id(dev_id){
        Ok(dev) => dev,
        Err(e) => panic!("{:?}", e)
    };

    println!("----- Iterator example usage: direct looping (preferred) -----");
    let cap_iterator = device.static_capabilities_iterator();
        for cap in cap_iterator {
            println!("[{}: {}]", cap.0, cap.1);
        }

    println!("----- Iterator example usage: explicit next call -----");
    // This time we'll use a virtual capabilities iterator
    let mut vcap_iterator = device.virtual_capabilities_iterator();
    let mut vcap = vcap_iterator.next();
    let mut has_next = vcap.is_some();
    while has_next {
        let vcap_v = vcap.unwrap();
        println!("[{}: {}]", vcap_v.0, vcap_v.1);
        vcap = vcap_iterator.next();
        has_next = vcap.is_some();
    }
}