use super::{request::getwebpage, clash::build_local_proxy_yaml};

pub fn check_bili_area(node: &serde_yaml::Value) -> Option<Vec<Country>> {
    fn check_main(node: &serde_yaml::Value) -> Option<Country> {
        match build_local_proxy_yaml(node) {
            Ok(_) => (),
            Err(_) => return None,
        }
        let raw_data = getwebpage("http://api.bilibili.com/x/web-interface/zone", "socks5h://127.0.0.1:2670", "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011", "")?;
        // println!("raw_data: {}", raw_data);
        let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&raw_data){
            value
        }else{
            return None;
        };
        if json_data["code"].as_i64().unwrap_or(2333) != 0 {
            return None;
        }
        return Some(Country::from_country_code_i64(json_data["data"]["country_code"].as_i64().unwrap_or(0)));
    }
    fn check_th(node: &serde_yaml::Value) -> Option<Country> {
        match build_local_proxy_yaml(node) {
            Ok(_) => (),
            Err(_) => return None,
        }
        let raw_data = getwebpage("http://ip-api.com/json/?fields=status,countryCode,query", "socks5h://127.0.0.1:2670", "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011", "")?;
        // println!("raw_data: {}", raw_data);
        let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&raw_data){
            value
        }else{
            return None;
        };
        if json_data["status"].as_str().unwrap() != "success" {
            return None;
        }
        return Some(Country::from_country_code_str(json_data["countryCode"].as_str().unwrap_or("")));
    }
    let mut countrys = Vec::with_capacity(2);
    if let Some(value) = check_main(node) {
        countrys.push(value);
    }
    if let Some(value) = check_th(node) {
        match value {
            Country::China => (),
            Country::Taiwan => (),
            Country::Hongkang => (),
            Country::Unknown => (),
            _ => {
                countrys.push(value);
            }
        }
    }
    if countrys.len() != 0 {
        return Some(countrys);
    }else{
        return None;
    }
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
    fn from_country_code_i64(country_code: i64) -> Self {
        match country_code {
            86 => Self::China,
            976 => Self::Mongolia,
            886 => Self::Taiwan,
            852 => Self::Hongkang,
            65 => Self::Singapore,
            _ => Self::Unknown,
        }
    }

    fn from_country_code_str(country_code: &str) -> Self {
        match country_code {
            "CN" => Self::China,
            "MN" => Self::Mongolia,
            "TW" => Self::Taiwan,
            "HK" => Self::Hongkang,
            "SG" => Self::Singapore,
            _ => Self::Unknown,
        }
    }
}