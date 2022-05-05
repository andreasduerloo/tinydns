use std::collections::HashMap;
use tinydns::{cache,ip};

fn main() {
    let mut cache: HashMap<String, ip::IPv4> = cache::setup();

    // DEBUG
    let an_ip: Option<ip::IPv4> = ip::IPv4::new_from_str("192.168.0.1");

    if let Some(valid_ip) = an_ip {
        cache::add_record(&mut cache, String::from("entry.com"), valid_ip);
        println!("Added first entry");
    } else {
        println!("Not a valid IP.");
    }

    let another_ip: Option<ip::IPv4> = ip::IPv4::new_from_str("192.168.256.1");

    if let Some(valid_ip) = another_ip {
        cache::add_record(&mut cache, String::from("secondentry.com"), valid_ip);
        println!("Added first entry");
    } else {
        println!("Not a valid IP.");
    }
}
