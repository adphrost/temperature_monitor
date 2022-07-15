use std::f64;
use std::process::{Command, Output};
// use mysql::*;
// use mysql::prelude::*;
use thiserror;
use std::result::Result;
// use std::error::Error;
use log;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread::sleep;

#[derive(Debug, thiserror::Error)]
pub enum BeepError {
    #[error("Failed to convert stdout (vec<u8>) to &str: {0:?}")]
    ParseStdoutError(Vec<u8>),
    #[error("Failed to convert &str to floating point number: {0}")]
    ConvertStringF64Error(String)
}





fn main() -> () {

    // let url = "mysql://adfrost:beepboop4@localhost:3306/temperature";
    // let pool = Pool::new(url)?;

    // let mut conn = pool.get_conn()?;
    println!("hi");
    loop {
        let cat_output = Command::new("cat")
            .arg("/sys/bus/w1/devices/28-01212e342610/temperature")
            .output();

            // gracefully ignore error so collector does not crash
            // something went wrong running cat

            let res: Output = match cat_output {
                Ok(res) => res,
                Err(_) => {
                    log::warn!("Failed to convert string temp to f64");
                    continue;  
                }
            };

            // gracefully ignore error so collector does not crash
            // maybe probe returned no temperature, assume zero
            let temperature: f64 = match parse_str_temperature(&res.stdout) {
                Some(t) => t,
                None => continue,
            };
        
            println!("temp is {} C", temperature);
            
            sleep(Duration::from_secs(5));
        // let now_u64: u64 = SystemTime::now()
            // .duration_since(UNIX_EPOCH)
            // .expect("Time went backwards")
            // .as_secs() as u64;
        // let val_13 = conn.exec_first(&stmt, params! { "foo" => 13, "bar" => foo })?.unwrap();
        // conn.exec(r"INSERT INTO temperature (temp, time) values (:temp, :now)", params! { "temp" => res_int, "now" => now_u64 })?.unwrap();
    }
}


fn parse_str_temperature(stdout: &Vec<u8>) -> Option<f64> {

    let temp_s: &str = match std::str::from_utf8(stdout) {
        Ok(t) => t.strip_suffix("\n").unwrap_or(t),
        Err(_) => {
            log::warn!("Failed to convert stdout to &str: {:?}",stdout.to_owned());
            return None;
        }
    };
    match temp_s.parse::<f64>() {
        // round to second decimal place
        Ok(t) => Some((t / 10.0).round() / 100.0),
        Err(_) => {
            log::warn!("Failed to convert string temperature to f64");
            None
        }
    }
}