use async_channel::{Receiver, Sender};
use bili_sub_filter::mods::{
    check::check_bili_area,
    clash::{build_delay_yaml, start_clash, get_delay_nodes},
    get_proxy::get_proxy_list,
    types::Config, request::get_node_delay,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, fs::File, sync::{Arc, Mutex}, time::Duration};
use tokio::{join, time::sleep};

lazy_static! {
    pub static ref CONFIG: Config =
        serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    pub static ref DELAY_CHANNEL: (Sender<u8>, Receiver<u8>) = async_channel::bounded(10);
    pub static ref DELAY_SENDER: Arc<Sender<u8>> = Arc::new(DELAY_CHANNEL.0.clone());
    pub static ref DELAY_RECEIVER: Arc<Receiver<u8>> = Arc::new(DELAY_CHANNEL.1.clone());
    pub static ref DELAY_RST_CHANNEL: (
        Sender<Arc<HashMap<serde_yaml::Value, bool>>>,
        Receiver<Arc<HashMap<serde_yaml::Value, bool>>>
    ) = async_channel::bounded(10);
    pub static ref DELAY_RST_SENDER: Arc<Sender<Arc<HashMap<serde_yaml::Value, bool>>>> =
        Arc::new(DELAY_RST_CHANNEL.0.clone());
    pub static ref DELAY_RST_RECEIVER: Arc<Receiver<Arc<HashMap<serde_yaml::Value, bool>>>> =
        Arc::new(DELAY_RST_CHANNEL.1.clone());
}

#[tokio::main]
async fn main() {
    let clash = tokio::spawn(async move { start_clash() });

    let check_connectivity_fn = tokio::spawn(async move {
        // CLASH_SENDER.send(1).await.unwrap();
        sleep(Duration::from_secs(2)).await;
        loop {
            println!("[Debug] Start a new thread, type: 1");
            let config: Config =
                serde_json::from_reader(File::open("config.json").unwrap()).unwrap(); // 这样改配置就不需要重启了，每次检查都获取新配置
            let mut cn_nodes: serde_yaml::Value = serde_yaml::from_str("proxies: []").unwrap();
            let mut hk_nodes = cn_nodes.clone();
            let mut tw_nodes = cn_nodes.clone();
            let mut th_nodes = cn_nodes.clone();
            let mut sg_nodes = cn_nodes.clone();
            let mut mn_nodes = cn_nodes.clone();
            let mut proxy_list: Vec<serde_yaml::Value> = Vec::with_capacity(200); //节点比较多
            let test_delay: bool;
            for sub in config.subs {
                let mut add_proxy_list = if let Some(value) = get_proxy_list(&sub) {
                    value
                } else {
                    continue;
                };
                proxy_list.append(&mut add_proxy_list);
            }
            if let Err(_) = build_delay_yaml(&proxy_list) {
                test_delay = false;
                println!("[Info] build_delay_yaml failed");
            } else {
                test_delay = true;
                DELAY_SENDER.send(1).await.unwrap();
            };
            for proxy_node in proxy_list {
                if let Some(countrys) = check_bili_area(&proxy_node) {
                    for country in countrys {
                        println!("{} -> {}", proxy_node["name"].as_str().unwrap(), country);
                        match country {
                            bili_sub_filter::mods::check::Country::China => {
                                cn_nodes["proxies"]
                                    .as_sequence_mut()
                                    .unwrap()
                                    .push(proxy_node.clone());
                            }
                            bili_sub_filter::mods::check::Country::Taiwan => {
                                tw_nodes["proxies"]
                                    .as_sequence_mut()
                                    .unwrap()
                                    .push(proxy_node.clone());
                            }
                            bili_sub_filter::mods::check::Country::Hongkang => {
                                hk_nodes["proxies"]
                                    .as_sequence_mut()
                                    .unwrap()
                                    .push(proxy_node.clone());
                            }
                            bili_sub_filter::mods::check::Country::Thailand => {
                                th_nodes["proxies"]
                                    .as_sequence_mut()
                                    .unwrap()
                                    .push(proxy_node.clone());
                            }
                            bili_sub_filter::mods::check::Country::Singapore => {
                                sg_nodes["proxies"]
                                    .as_sequence_mut()
                                    .unwrap()
                                    .push(proxy_node.clone());
                            }
                            bili_sub_filter::mods::check::Country::Mongolia => {
                                mn_nodes["proxies"]
                                    .as_sequence_mut()
                                    .unwrap()
                                    .push(proxy_node.clone());
                            }
                            bili_sub_filter::mods::check::Country::Unknown => (),
                        }
                    }
                }
            }
            if test_delay {
                if let Ok(delay_map) = DELAY_RST_RECEIVER.recv().await {
                    let retain_fn = |a: &serde_yaml::Value| {
                        if *delay_map.get(&a["name"]).unwrap_or(&false) {
                            true
                        } else {
                            false
                        }
                    };
                    cn_nodes.as_sequence_mut().unwrap().retain(retain_fn);
                    tw_nodes.as_sequence_mut().unwrap().retain(retain_fn);
                    hk_nodes.as_sequence_mut().unwrap().retain(retain_fn);
                    th_nodes.as_sequence_mut().unwrap().retain(retain_fn);
                    sg_nodes.as_sequence_mut().unwrap().retain(retain_fn);
                    mn_nodes.as_sequence_mut().unwrap().retain(retain_fn);
                } else {
                    continue;
                };
            }
            serde_yaml::to_writer(File::create("./output/cn.yaml").unwrap(), &cn_nodes)
                .unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/tw.yaml").unwrap(), &tw_nodes)
                .unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/hk.yaml").unwrap(), &hk_nodes)
                .unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/th.yaml").unwrap(), &th_nodes)
                .unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/sg.yaml").unwrap(), &sg_nodes)
                .unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/mn.yaml").unwrap(), &mn_nodes)
                .unwrap_or_default();
            println!("fi");
            sleep(Duration::from_secs(60 * 60)).await;
        }
    });

    let check_delay_fn = tokio::spawn(async move {
        loop {
            let channel_code = if let Ok(value) = DELAY_RECEIVER.recv().await {
                value
            } else {
                continue;
            };
            match channel_code {
                1 => {
                    let delay_map: HashMap<serde_yaml::Value, bool> = HashMap::new();
                    let mutex_delay_map = Arc::new(Mutex::new(delay_map));
                    match get_delay_nodes().await {
                        Some(nodes) => {
                            let num_of_nodes = nodes.len();
                            let mutex_nodes = Arc::new(Mutex::new(nodes));
                            let mut handles = Vec::with_capacity(num_of_nodes);
                            for _i in 0..num_of_nodes {
                                let new_mutex_delay_map = mutex_delay_map.clone();
                                let new_mutex_nodes = mutex_nodes.clone();
                                let check = tokio::spawn(async move {
                                    let node = {
                                        if let Some(value) = new_mutex_nodes.lock().unwrap().pop() {
                                            value
                                        }else{
                                            return;
                                        }
                                    };
                                    let delay = get_node_delay(&node.as_str().unwrap(),"JCasbciSCBAISw").await;
                                    let ok = {
                                        if delay == 0 || delay > 1000 {
                                            false
                                        }else{
                                            true
                                        }
                                    };
                                    new_mutex_delay_map.lock().unwrap().insert(serde_yaml::Value::String(node.as_str().unwrap().to_string()),ok);
                                });
                                handles.push(check);
                            }
                            for handle in handles {
                                handle.await.unwrap_or_default();
                            }
                        },
                        None => {
                            DELAY_RST_SENDER.send(Arc::new(HashMap::new())).await.unwrap();
                        }
                    }
                    // for 
                }
                _ => {
                    panic!("unknown channel code")
                }
            }
        }
    });

    join!(clash, check_connectivity_fn, check_delay_fn)
        .1
        .unwrap_or_default();
}
