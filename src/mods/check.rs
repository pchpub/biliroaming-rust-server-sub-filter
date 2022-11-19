use super::{request::getwebpage, clash::build_local_proxy_yaml};

pub fn check_bili_area(node: &serde_yaml::Value) -> Option<Country> {
    //  b站api：http://api.bilibili.com/x/web-interface/zone
    //          http://api.bilibili.com/client_info
    match build_local_proxy_yaml(node) {
        Ok(_) => (),
        Err(_) => return None,
    }
    let raw_data = getwebpage("http://api.bilibili.com/x/web-interface/zone", "socks5h://127.0.0.1:2670", "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011", "")?;
    println!("1");
    let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&raw_data){
        value
    }else{
        return None;
    };
    if json_data["code"].as_i64().unwrap_or(2333) != 0 {
        return None;
    }
    return Some(Country::new(json_data["data"]["country_code"].as_i64().unwrap_or(0)));
}

pub enum Country{
    China,
    Taiwan,// TW
    Hongkang,// HK
    Thailand,// South East Asia
    Singapore,// South East Asia
    Mongolia, // South East Asia
    Unknown, // unknown
}

impl Country {
    fn new(country_code: i64) -> Self {
        match country_code {
            86 => Self::China,
            976 => Self::Mongolia,
            886 => Self::Taiwan,
            852 => Self::Hongkang,
            65 => Self::Singapore,
            _ => Self::Unknown,
        }
    }
}