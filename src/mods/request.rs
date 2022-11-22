use curl::easy::{Easy, List};
use std::{collections::HashMap, time::Duration};
use tokio::task::spawn_blocking;

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
        headers_list
            .append(&format!("Authorization: Bearer {authorization}"))
            .unwrap();
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
                println!("[Debug] getwebpage error{}", value);
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

pub async fn async_getwebpage(
    url: &str,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    timeout: &Duration,
    authorization: &str,
) -> Option<String> {
    let url = url.to_owned();
    let proxy_url = proxy_url.to_owned();
    let user_agent = user_agent.to_owned();
    let cookie = cookie.to_owned();
    let timeout = timeout.to_owned();
    let authorization = authorization.to_owned();
    match spawn_blocking(move || {
        getwebpage(
            &url,
            &proxy_url,
            &user_agent,
            &cookie,
            &timeout,
            &authorization,
        )
    })
    .await
    {
        Ok(value) => value,
        _ => return None,
    }
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
    headers_list
        .append(&format!("Authorization: Bearer {authorization}"))
        .unwrap();
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
                println!("[Debug] update_proxy_provider error:{}", value);
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

pub async fn get_node_delay(node_name: &str, authorization: &str) -> u64 {
    let raw_data = if let Some(value) = getwebpage(
        &format!("http://127.0.0.1:2671/proxies/{}/delay", node_name),
        "",
        "",
        "",
        &Duration::from_secs(60),
        authorization,
    ) {
        value
    } else {
        return 0;
    };
    let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&raw_data) {
        value
    } else {
        return 0;
    };
    return json_data["delay"].as_u64().unwrap_or(0);
}

pub async fn get_nodes_delay() -> Option<HashMap<String, u64>> {
    let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&async_getwebpage("http://127.0.0.1:2671/group/delay/delay?url=http://www.gstatic.com/generate_202&timeout=20000", "", "", "", &Duration::from_secs(60), "JCasbciSCBAISw").await.unwrap_or_default()){
        value
    }else{
        return None;
    };
    return Some(
        json_data
            .as_object()
            .unwrap()
            .iter()
            .map(|a| (a.0.clone(), a.1.as_u64().unwrap_or(0)))
            .collect(),
    );
}
