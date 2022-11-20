use std::time::Duration;

use curl::easy::{Easy, List};

pub fn getwebpage(
    url: &str,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
) -> Option<String> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(false).unwrap();
    handle.useragent(user_agent).unwrap();
    handle.connect_timeout(Duration::new(5, 0)).unwrap();
    handle.cookie(cookie).unwrap();
    handle.proxy(proxy_url).unwrap();

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
    dnt: &i32,
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
    headers_list.append(&format!("DNT: {dnt}")).unwrap();
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