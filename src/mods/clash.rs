use std::{fs::File, process::Command, time::Duration, collections::HashMap};

use tokio::task::spawn_blocking;

use super::request::getwebpage;

pub async fn start_clash(config_path: &str) -> Result<String, ()> {
    let config_path = config_path.to_owned();
    match spawn_blocking(move || {
        let output = if let Ok(value) = Command::new("./clash/clash")
            .arg("-d")
            .arg(&config_path)
            .arg(">")
            .arg("/dev/null")
            .arg("&")
            .arg("echo")
            .arg("$!")
            .output()
        {
            value
        } else {
            return Err(());
        };
        let output = String::from_utf8(output.stdout).unwrap_or("".to_owned());
        println!("pid: {}", output);
        return Ok(output);
    })
    .await
    {
        Ok(value) => value,
        _ => Err(()),
    }
    
}

pub fn build_connectivity_yaml(node: &serde_yaml::Value) -> Result<(), ()> {
    let raw_data = r#"proxies: null"#;
    let mut yaml_data: serde_yaml::Value = serde_yaml::from_str(raw_data).unwrap();
    yaml_data["proxies"] = serde_yaml::Value::Sequence(vec![node.clone()]);
    // yaml_data["proxy-groups"][0]["proxies"] =
    //     serde_yaml::Value::Sequence(vec![node["name"].clone()]); //node["name"].clone();
    let yaml_file = File::create("./clash/test-connectivity.yaml").unwrap();
    match serde_yaml::to_writer(yaml_file, &yaml_data) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn build_delay_yaml(nodes: &Vec<serde_yaml::Value>) -> Result<(), ()> {
    let raw_data = r#"proxies: null"#;
    let mut yaml_data: serde_yaml::Value = serde_yaml::from_str(raw_data).unwrap();
    yaml_data["proxies"] = serde_yaml::Value::Sequence(nodes.clone());
    // yaml_data["proxy-groups"][0]["proxies"] =
    //     serde_yaml::Value::Sequence(nodes.iter().map(|a| a["name"].clone()).collect());
    let yaml_file = File::create("./clash/test-delay.yaml").unwrap();
    match serde_yaml::to_writer(yaml_file, &yaml_data) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub async fn get_delay_nodes() -> Option<HashMap<String,u64>> {
    let json_data: serde_json::Value = if let Ok(value) = serde_json::from_str(&getwebpage("http://127.0.0.1:2671/group/delay/delay?url=http://www.gstatic.com/generate_202&timeout=20000", "", "", "", &Duration::from_secs(60), "JCasbciSCBAISw").unwrap_or_default()){
        value
    }else{
        return None;
    };
    return Some(json_data.as_object().unwrap().iter().map(|a|{
        (a.0.clone(),a.1.as_u64().unwrap_or(0))
    }).collect());
}