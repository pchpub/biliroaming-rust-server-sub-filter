use std::time::Duration;

use curl::easy::Easy;

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
                println!("{}",value);
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