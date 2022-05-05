pub mod cache {
    use std::collections::HashMap;
    use crate::ip;

    pub fn setup() -> HashMap<String, ip::IPv4> {
        let emptymap: HashMap<String, ip::IPv4> = HashMap::new();
        emptymap
    }

    pub fn add_record(cache: &mut HashMap<String, ip::IPv4>, new_name: String, new_address: ip::IPv4) {
        let _ = cache.insert(new_name, new_address);
    }

    pub fn parse(cache: &mut HashMap<String, ip::IPv4>, forward: &mut ip::IPv4, config: Vec<&str>) {
        for line in config {
            if &line[..1] == "#" {
                println!("That's a comment");
                continue
            } else if &line[..7] == "FORWARD" {
                let splitline: Vec<&str> = line.split_whitespace().collect();

                let forward_ip = ip::IPv4::new_from_str(splitline[1]);

                match forward_ip {
                    Ok(an_ip) => {
                        *&mut forward.address = an_ip.address;
                        println!("Forwarding IP set")
                    },
                    Err(_) => {
                        println!("Incorrect forwarding IPv4 in config file")
                    }
                };
            } else {
                let splitline: Vec<&str> = line.split_whitespace().collect();

                let config_ip = ip::IPv4::new_from_str(splitline[1]);

                match config_ip {
                    Ok(an_ip) => {
                        let name: String = String::from(splitline[0]);
                        add_record(cache, name, an_ip);
                        println!("Entry added")
                    },
                    Err(_) => {
                        println!("Incorrect entry in config file")
                    }
                };
            }
        }
    }
}

pub mod ip {
    pub struct IPv4 {
        pub address: [u8; 4],
    }

    impl IPv4 {
        pub fn new(address: [u8; 4]) -> IPv4 {
            IPv4 {
                address
            }
        }
    }

    impl IPv4 {
        pub fn new_from_str(ip_string: &str) -> Result<IPv4, &str> {
            let ip_vec: Vec<&str> = ip_string.split(".").collect();

            match ip_vec.len() {
                4 => {
                    let mut octets: [u8; 4] = [0; 4];
                    for octet in 0..4 {
                        let current_octet = u8::from_str_radix(ip_vec[octet], 10);

                        match current_octet {
                            Ok(valid_octet) => {
                                *&mut octets[octet] = valid_octet;
                            },
                            Err(_) => {
                                return Err("Invalid octet");
                            }
                        }
                    }

                    let out_ip: IPv4 = IPv4::new(octets);
                    Ok(out_ip)
                },
                _ => Err("Invalid IPv4")
            }
        }
    }
}
