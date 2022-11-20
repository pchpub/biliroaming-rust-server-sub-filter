use std::{fs::File, sync::Arc, time::Duration};
use async_channel::{Sender, Receiver};
use lazy_static::lazy_static;
use bili_sub_filter::mods::{types::Config, clash::start_clash, get_proxy::get_proxy_list, check::check_bili_area};
use tokio::{process::Command, time::sleep, join};

lazy_static! {
    pub static ref CONFIG: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    pub static ref CLASH_CHANNEL: (Sender<u8>,Receiver<u8>) = async_channel::bounded(10);
    pub static ref CLASH_SENDER: Arc<Sender<u8>> = Arc::new(CLASH_CHANNEL.0.clone());
    pub static ref CLASH_RECEIVER: Arc<Receiver<u8>> = Arc::new(CLASH_CHANNEL.1.clone());
}


#[tokio::main]
async fn main() {
    // TODO 订阅筛选
    // println!("Hello, world!");
    let clash = tokio::spawn( async move {
        println!("[Debug] Start a new thread, type: 2");
        let mut pid = None;
        loop {
            let channel_code = if let Ok(value) = CLASH_RECEIVER.recv().await {
                value
            }else{
                continue;
            };
            match channel_code {
                1 => {
                    pid = if let Ok(value) = start_clash(){
                        Some(value.clone())
                    }else{
                        None
                    };
                    println!("pid th-1: {:?}", pid);
                },
                0 => {
                    match pid.clone() {
                        Some(value) => {
                            match Command::new("kill").arg("-9").arg(&value).output().await {
                                Ok(_) => {
                                    pid = None;
                                },
                                Err(_) => {
                                    println!("[Debug] failed to kill clash");
                                },
                            };
                        },
                        None => {()},
                    }
                    // Command::new("kill").arg("-9").arg().arg("echo").arg("$!").output()
                },
                _ => {panic!("Unexpected channel code")},
            }
        }
    });

    let main_fn = tokio::spawn(async move {
        loop{
            println!("[Debug] Start a new thread, type: 1");
            let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap(); // 这样改配置就不需要重启了，每次检查都获取新配置
            let mut cn_nodes:serde_yaml::Value = serde_yaml::from_str("proxies: []").unwrap();
            let mut hk_nodes = cn_nodes.clone();
            let mut tw_nodes = cn_nodes.clone();
            let mut th_nodes = cn_nodes.clone();
            let mut sg_nodes = cn_nodes.clone();
            let mut mo_nodes = cn_nodes.clone();
            for sub in config.subs {
                let proxy_list = if let Some(value) = get_proxy_list(&sub) {
                    value
                }else{
                    continue;
                };
                for proxy_node in proxy_list {
                    CLASH_SENDER.send(1).await.unwrap();
                    sleep(Duration::from_secs(2)).await;
                    if let Some(countrys) = check_bili_area(&proxy_node) {
                        for country in countrys {
                            match country {
                                bili_sub_filter::mods::check::Country::China => {
                                    cn_nodes["proxies"].as_sequence_mut().unwrap().push(proxy_node.clone());
                                },
                                bili_sub_filter::mods::check::Country::Taiwan => {
                                    tw_nodes["proxies"].as_sequence_mut().unwrap().push(proxy_node.clone());
                                },
                                bili_sub_filter::mods::check::Country::Hongkang => {
                                    hk_nodes["proxies"].as_sequence_mut().unwrap().push(proxy_node.clone());
                                },
                                bili_sub_filter::mods::check::Country::Thailand => {
                                    th_nodes["proxies"].as_sequence_mut().unwrap().push(proxy_node.clone());
                                },
                                bili_sub_filter::mods::check::Country::Singapore => {
                                    sg_nodes["proxies"].as_sequence_mut().unwrap().push(proxy_node.clone());
                                },
                                bili_sub_filter::mods::check::Country::Mongolia => {
                                    mo_nodes["proxies"].as_sequence_mut().unwrap().push(proxy_node.clone());
                                },
                                bili_sub_filter::mods::check::Country::Unknown => (),
                            }
                        }
                    }
                    CLASH_SENDER.send(0).await.unwrap();
                }
            }
            serde_yaml::to_writer(File::create("./output/cn.yaml").unwrap(),&cn_nodes).unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/tw.yaml").unwrap(),&tw_nodes).unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/hk.yaml").unwrap(),&hk_nodes).unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/th.yaml").unwrap(),&th_nodes).unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/sg.yaml").unwrap(),&sg_nodes).unwrap_or_default();
            serde_yaml::to_writer(File::create("./output/mo.yaml").unwrap(),&mo_nodes).unwrap_or_default();
            sleep(Duration::from_secs(20*60)).await;
        }
    });

    join!(clash,main_fn).1.unwrap_or_default();
}
