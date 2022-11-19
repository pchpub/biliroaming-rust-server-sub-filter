use std::{process::Command, fs::File};

pub fn start_clash() -> Result<String,()>{
    let output = if let Ok(value) = Command::new("nohup").arg("./clash/clash").arg("-d").arg(".").arg("&").arg("echo").arg("$!").output() {
        value
    }else{
        return Err(());
    };
    let ls_la_list = String::from_utf8(output.stdout).unwrap_or("".to_owned());
    return Ok(ls_la_list);
}

pub fn build_local_proxy_yaml(node: &serde_yaml::Value) -> Result<(),()> {
    let raw_data = 
r#"allow-lan: false
bind-address: '*'
dns:
  default-nameserver:
  - 223.5.5.5
  - 119.29.29.29
  enable: true
  enhanced-mode: redir-host
  fake-ip-range: 198.18.0.1/16
  fallback:
  - tls://1.0.0.1:853
  - https://cloudflare-dns.com/dns-query
  - https://dns.google/dns-query
  fallback-filter:
    geoip: true
    ipcidr:
    - 240.0.0.0/4
    - 0.0.0.0/32
  ipv6: false
  nameserver:
  - https://doh.pub/dns-query
  - https://dns.alidns.com/dns-query
  use-hosts: true
external-controller: 127.0.0.1:1123
ipv6: false
log-level: info
mixed-port: 1122
mode: rule
proxies: 
- {name: test-node}
proxy-groups:
- name: auto
  type: select
  proxies: 
    - test-node

rules:
- DOMAIN-SUFFIX,google.com,auto
- DOMAIN-KEYWORD,google,auto
- DOMAIN,google.com,auto
- SRC-IP-CIDR,192.168.1.201/32,DIRECT
- IP-CIDR,127.0.0.0/8,DIRECT
- GEOIP,CN,DIRECT
- DST-PORT,80,DIRECT
- SRC-PORT,7777,DIRECT
- MATCH,auto"#;
    println!("{}",raw_data);
    let mut yaml_data: serde_yaml::Value = serde_yaml::from_str(raw_data).unwrap();
    yaml_data["proxies"].as_sequence_mut().unwrap().push(node.to_owned());
    let yaml_file = File::create("./clash/proxy.yaml").unwrap();
    match serde_yaml::to_writer(yaml_file, &yaml_data){
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}