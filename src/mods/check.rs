use std::time::Duration;

use crate::mods::request::update_proxy_provider;

use super::{clash::build_connectivity_yaml, request::getwebpage};

pub fn check_bili_area(node: &serde_yaml::Value) -> Option<Vec<Country>> {
    fn check_main() -> Option<Country> {
        let raw_data = getwebpage(
            "http://api.bilibili.com/x/web-interface/zone",
            "socks5h://127.0.0.1:2670",
            "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011",
            "",
            &Duration::from_secs(2),
            "",
        )?;
        // println!("raw_data: {}", raw_data);
        let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&raw_data) {
            value
        } else {
            return None;
        };
        if json_data["code"].as_i64().unwrap_or(2333) != 0 {
            return None;
        }
        return Some(Country::from_country_code_i64(
            json_data["data"]["country_code"].as_i64().unwrap_or(0),
        ));
    }
    fn check_th(node: &serde_yaml::Value) -> Option<Country> {
        let raw_data = getwebpage(
            "http://ip-api.com/json/?fields=status,countryCode,query",
            "socks5h://127.0.0.1:2670",
            "Dalvik/2.1.0 (Linux; U; Android 11; 21091116AC Build/RP1A.200720.011",
            "",
            &Duration::from_secs(1),
            "",
        )?;
        println!(
            "{} -> ip-api raw_data: {}",
            node["name"].as_str().unwrap(),
            raw_data
        );
        let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&raw_data) {
            value
        } else {
            return None;
        };
        if json_data["status"].as_str().unwrap() != "success" {
            return None;
        }
        return Some(Country::from_country_code_str(
            json_data["countryCode"].as_str().unwrap_or(""),
        ));
    }
    match build_connectivity_yaml(node) {
        Ok(_) => (),
        Err(_) => return None,
    }
    update_proxy_provider(
        "http://127.0.0.1:2671/providers/proxies/TestConnectivity",
        "",
        "",
        "",
        "JCasbciSCBAISw",
    )
    .unwrap_or_default();
    let mut countrys = Vec::with_capacity(2);
    if let Some(value) = check_main() {
        match value {
            Country::China => countrys.push(value),
            Country::Taiwan => countrys.push(value),
            Country::Hongkang => countrys.push(value),
            _ => (),
        }
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
    } else {
        return None;
    }
}

pub enum Country {
    China,
    Taiwan,    // TW
    Hongkang,  // HK
    Thailand,  // South East Asia
    Singapore, // South East Asia
    Mongolia,  // South East Asia
    Unknown,   // unknown
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
            "TH" => Self::Thailand,
            _ => Self::Unknown,
        }
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Country::China => {
                write!(f, "cn")
            }
            Country::Taiwan => {
                write!(f, "tw")
            }
            Country::Hongkang => {
                write!(f, "hk")
            }
            Country::Thailand => {
                write!(f, "th")
            }
            Country::Singapore => {
                write!(f, "sg")
            }
            Country::Mongolia => {
                write!(f, "mn")
            }
            Country::Unknown => {
                write!(f, "un")
            }
        }
    }
}
