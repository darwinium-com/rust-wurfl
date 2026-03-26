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
use wurfl::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{SystemTime, Duration};


// This example is a simple program that performs a massive device detection using a WURFL engine.
// It reads a test dataset file specified by command line argument,
// and measures the average time taken to detect the device for each user agent in the dataset.
// The dataset can be a plain text file with one user agent per line, or a tab-separated file with
// user agent as the first column.
// The results are printed to the console.
//
// This example assumes that the WURFL file is located in the current directory, and tries to download
// an uppdated version of the WURFL file from the ScientiaMobile Snapshot generator. 
// Replace the sample URL with your own specific customer URL to avoid error 402.

// Example usage: cargo run --release --example massive_detection_test /path/to/dataset/file.txt

fn main() {

    let mut test_file_path = "".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        test_file_path = args[1].clone();
    }

    
    // download WURFL file from the ScientiaMobile Snapshot generator (replace the sample URL with your own specific customer URL to avoid error 402)
    let download_res = wurfl_download("http://data.scientiamobile.com/xxxxx/wurfl.zip", ".");
    match download_res {
        Ok(_) => println!("WURFL file downloaded successfully"),
        Err(we) => panic!("{}", we.msg)
    }
    let wurfl_path = "test_files/wurfl.zip";
    // empty patches
    let patches = None;
    // empty capability filter
    let filter = None;
    let wurfl_res = Wurfl::new(wurfl_path, patches, filter,
                               WurflCacheProvider::LRU, Some("200000"));
    let engine = match wurfl_res {
        Ok(engine) => engine,
        Err(we) => panic!("{}", we.msg)
    };

    
    let is_tsv = test_file_path.ends_with(".tsv");
    if let Ok(lines) = read_lines(test_file_path) {
        let mut line_count = 0;
        let mut total_detection_time = Duration::new(0, 0);

        for line in lines {
            let ua = extract_user_agent(line.unwrap(), is_tsv);

            let start = SystemTime::now();
            let device = engine.lookup_useragent(ua.as_str());
            let stop = SystemTime::now();
            if device.is_err(){
                panic!("lookup user agent failed: test massive detection test aborted")
            }
            let diff = stop.duration_since(start);
            if diff.is_ok(){
                total_detection_time = total_detection_time + diff.unwrap();
                line_count += 1;
            } else {
                panic!("Diff calculation failed, aborting test")
            }
        }
        let avg_detection_time_in_micros = total_detection_time.as_micros() as f32 / line_count as f32;
        println!("###############################################################################");
        println!("Processed {} lines", line_count);
        println!("Average detection time in microseconds: {}", avg_detection_time_in_micros);
        println!("###############################################################################");
    }


    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn extract_user_agent(line: String, is_tsv: bool) -> String {
        return if is_tsv {
            let mut split_line = line.split("\t");
            let first_line_token = split_line.next();
            if first_line_token.is_none() {
                "".to_owned()
            } else {
                first_line_token.unwrap().to_owned()
            }
        } else {
            line.to_owned()
        }
    }
}