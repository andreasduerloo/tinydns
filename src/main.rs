use std::collections::HashMap;
use tinydns::{cache,ip};

fn main() {
    let mut cache: HashMap<String, ip::IPv4> = cache::setup();

    // DEBUG
    let an_ip = ip::IPv4::new_from_str("192.168.0.1");

    let _ = match an_ip {
        Ok(valid_ip) => {
            cache::add_record(&mut cache, String::from("entry.com"), valid_ip);
            println!("Success! Added A-record")
        },
        Err(_) => {
            println!("Invalid IPv4 in configuration");
        }
    };

    let another_ip = ip::IPv4::new_from_str("192.168.0.1.1"); // Too long

    let _ = match another_ip {
        Ok(valid_ip) => {
            cache::add_record(&mut cache, String::from("secondentry.com"), valid_ip);
            println!("Success! Added A-record")
        },
        Err(_) => {
            println!("Invalid IPv4 in configuration");
        }
    };

    let final_ip = ip::IPv4::new_from_str("192.256.0.A"); // Can't be a u8

    let _ = match final_ip {
        Ok(valid_ip) => {
            cache::add_record(&mut cache, String::from("finalentry.com"), valid_ip);
            println!("Success! Added A-record")
        },
        Err(_) => {
            println!("Invalid IPv4 in configuration");
        }
    };



    // if let Some(valid_ip) = an_ip {
    //     cache::add_record(&mut cache, String::from("entry.com"), valid_ip);
    //     println!("Added first entry");
    // } else {
    //     println!("Not a valid IP.");
    // }

    // let another_ip: Option<ip::IPv4> = ip::IPv4::new_from_str("192.168.256.1");

    // if let Some(valid_ip) = another_ip {
    //     cache::add_record(&mut cache, String::from("secondentry.com"), valid_ip);
    //     println!("Added second entry");
    // } else {
    //     println!("Not a valid IP.");
    // }

    // let final_ip: Option<ip::IPv4> = ip::IPv4::new_from_str("hello.192.168.256.1");

    // if let Some(valid_ip) = final_ip {
    //     cache::add_record(&mut cache, String::from("finalentry.com"), valid_ip);
    //     println!("Added final entry");
    // } else {
    //     println!("Not a valid IP.");
    // }
}
