use std::time::Duration;
use curl::easy::{Easy, List};

pub fn getwebpage(
    url: &str,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    timeout: &Duration,
    authorization: &str,
) -> Option<String> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(false).unwrap();
    handle.useragent(user_agent).unwrap();
    handle.connect_timeout(*timeout).unwrap();
    handle.cookie(cookie).unwrap();
    handle.proxy(proxy_url).unwrap();
    if authorization.len() != 0 {
        let mut headers_list = List::new();
        headers_list.append(&format!("Authorization: Bearer {authorization}")).unwrap();
        handle.http_headers(headers_list).unwrap();
    }

    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        match transfer.perform() {
            Ok(()) => (),
            Err(value) => {
                println!("[Debug] getwebpage error{}",value);
                return None;
            }
        }
    }

    let getwebpage_string: String = match String::from_utf8(data) {
        Ok(value) => value,
        Err(_) => {
            return None;
        }
    };
    return Some(getwebpage_string);
}

pub fn update_proxy_provider(
    url: &str,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    authorization: &str,
) -> Option<String> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut headers_list = List::new();
    handle.url(url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(false).unwrap();
    handle.upload(true).unwrap();
    handle.useragent(user_agent).unwrap();
    handle.connect_timeout(Duration::new(5, 0)).unwrap();
    handle.cookie(cookie).unwrap();
    handle.proxy(proxy_url).unwrap();
    headers_list.append("Content-Length: 0").unwrap();
    headers_list.append(&format!("Authorization: Bearer {authorization}")).unwrap();
    handle.http_headers(headers_list).unwrap();

    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        match transfer.perform() {
            Ok(()) => (),
            Err(value) => {
                println!("[Debug] update_proxy_provider error:{}",value);
                return None;
            }
        }
    }

    let putwebpage_string: String = match String::from_utf8(data) {
        Ok(value) => value,
        Err(_) => {
            return None;
        }
    };
    // println!("{}",putwebpage_string);
    return Some(putwebpage_string);
}

pub async fn get_node_delay(node_name: &str,authorization: &str) -> u64 {
    let raw_data = if let Some(value) = getwebpage(&format!("http://127.0.0.1:2671/proxies/{}/delay",node_name), "", "", "", &Duration::from_secs(60), authorization){
        value
    }else{
        return 0;
    };
    let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&raw_data) {
        value
    }else{
        return 0;
    };
    return json_data["delay"].as_u64().unwrap_or(0);
}

