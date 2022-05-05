use std::collections::HashMap;
use tinydns::{cache,ip};
use std::{env,fs};

fn main() {
    // Initialize and fill cache
    let mut cache: HashMap<String, ip::IPv4> = cache::setup();
    let mut forward_to: ip::IPv4 = ip::IPv4::new([1, 1, 1, 1]);

    // Collect the arguments
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("No config file - exiting");
            return
        },
        2 => {
            let configfile = fs::read_to_string(&args[1]);

            let configfile = match configfile {
                Ok(content) => content,
                Err(_) => {
                    println!("Error opening file");
                    return
                }
            };
            
            let configlines: Vec<&str> = configfile.lines().collect();

            cache::parse(&mut cache, &mut forward_to, configlines);
        },
        _ => {
            println!("Incorrect arguments - exiting");
            return
        }
    }

    // Test

    let response = cache.get("gateway.local");

    if let Some(target_address) = response {
        println!("gateway.local is at {}.{}.{}.{}", target_address.address[0], target_address.address[1], target_address.address[2], target_address.address[3]);
    } else {
        println!("gateway.local not found")
    }
}
