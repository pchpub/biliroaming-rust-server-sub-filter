use std::{fs::File, process::Command};

pub fn start_clash() -> Result<String, ()> {
    // let output = if let Ok(value) = Command::new("nohup").arg("./clash/clash").arg("-d").arg("./clash/").arg(">").arg("/dev/null").arg("&").arg("echo").arg("$!").output() {
    let output = if let Ok(value) = Command::new("./clash/clash")
        .arg("-d")
        .arg("./clash/")
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
