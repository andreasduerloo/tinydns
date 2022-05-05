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
}

pub mod ip {
    pub struct IPv4 {
        pub address: [u8; 4],
    }

    impl IPv4 {
        fn new(address: [u8; 4]) -> IPv4 {
            IPv4 {
                address
            }
        }
    }

    impl IPv4 {
        pub fn new_from_str(ip_string: &str) -> Option<IPv4> {
            let ip_vec: Vec<&str> = ip_string.split(".").collect();

            match ip_vec.len() {
                4 => {
                    let mut octets: [u8; 4] = [0; 4];

                    for octet in 0..4 {
                        *&mut octets[octet] = u8::from_str_radix(ip_vec[octet], 10).expect("Unable to store IP adress: octet is not a u8")
                    }

                    let out_ip: IPv4 = IPv4::new(octets);
                    Some(out_ip)
                },
                _ => None
            }
        }
    }
}