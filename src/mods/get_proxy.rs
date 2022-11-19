use super::request::getwebpage;

fn get_proxy_provider(url: &str) -> Option<serde_yaml::Mapping> {
    let raw_data = getwebpage(url, "", "Clash/1.0", "cookie")?;
    let yaml_data: serde_yaml::Value = if let Ok(value) = serde_yaml::from_str(&raw_data) {
        value
    } else {
        return None;
    };
    if {
        if let Some(value) = yaml_data.as_mapping() {
            value
        } else {
            return None;
        }
    }
    .contains_key("proxies")
    {
        return Some(yaml_data.as_mapping().unwrap().clone());
    }
    None
}

pub fn get_proxy_list(url: &str) -> Option<Vec<serde_yaml::Value>> {
    let proxy_provider = get_proxy_provider(url)?;
    if proxy_provider.len() == 0 {
        return None;
    }
    let mut proxy_list = vec![];
    for proxy_node in proxy_provider["proxies"].as_sequence().unwrap().iter() {
        proxy_list.push(proxy_node.clone());
    }
    Some(proxy_list)
}

