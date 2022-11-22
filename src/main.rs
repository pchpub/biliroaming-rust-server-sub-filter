use bili_sub_filter::mods::{
    check::{check_bili_area, Country},
    clash::{build_delay_yaml, start_clash},
    get_proxy::get_proxy_list,
    request::{get_nodes_delay, update_proxy_provider},
    types::Config,
};
use lazy_static::lazy_static;
use std::{fs::File, time::Duration, sync::{Arc}, collections::HashMap};
use tokio::{join, time::sleep, sync::Mutex, task::JoinHandle};

lazy_static! {
    pub static ref CONFIG: Config =
        serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    // pub static ref DELAY_CHANNEL: (Sender<u8>, Receiver<u8>) = async_channel::bounded(10);
    // pub static ref DELAY_SENDER: Arc<Sender<u8>> = Arc::new(DELAY_CHANNEL.0.clone());
    // pub static ref DELAY_RECEIVER: Arc<Receiver<u8>> = Arc::new(DELAY_CHANNEL.1.clone());
    // pub static ref DELAY_RST_CHANNEL: (
    //     Sender<Arc<HashMap<serde_yaml::Value, bool>>>,
    //     Receiver<Arc<HashMap<serde_yaml::Value, bool>>>
    // ) = async_channel::bounded(10);
    // pub static ref DELAY_RST_SENDER: Arc<Sender<Arc<HashMap<serde_yaml::Value, bool>>>> =
    //     Arc::new(DELAY_RST_CHANNEL.0.clone());
    // pub static ref DELAY_RST_RECEIVER: Arc<Receiver<Arc<HashMap<serde_yaml::Value, bool>>>> =
    //     Arc::new(DELAY_RST_CHANNEL.1.clone());
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
                println!("-----60");
                update_proxy_provider(
                    "http://127.0.0.1:2671/providers/proxies/TestDelay",
                    "",
                    "",
                    "",
                    "JCasbciSCBAISw",
                )
                .unwrap_or_default();
                // DELAY_SENDER.send(1).await.unwrap();
            };
            println!("-----63");
            let mutex_proxy_list = Arc::new(Mutex::new(proxy_list));
            let mutex_proxy_rst_list: Arc<Mutex<HashMap<serde_yaml::Value, Vec<Country>>>> = Arc::new(Mutex::new(HashMap::new()));
            let mut check_area_handles = vec![];
            for _i in 0..10 { //10个同时
                let new_mutex_proxy_list = mutex_proxy_list.clone();
                let new_mutex_proxy_rst_list = mutex_proxy_rst_list.clone();
                let check: JoinHandle<Result<(),()>> = tokio::spawn(async move {
                    loop{
                        let node = {
                            if let Some(value) = new_mutex_proxy_list.lock().await.pop() {
                                value
                            }else{
                                return Ok(());
                            }
                        };
                        let countrys = if let Some(value) = check_bili_area(&node).await {
                            value
                        }else{
                            continue;
                        };
                        {
                            new_mutex_proxy_rst_list.lock().await.insert(node, countrys);
                        }
                    }
                });
                check_area_handles.push(check);
            }
            for handle in check_area_handles {
                handle.await.unwrap().unwrap_or_default();
            }
            for connect_rst in mutex_proxy_rst_list.lock().await.iter() {
                for country in connect_rst.1 {
                    println!("{} -> {}", connect_rst.0["name"].as_str().unwrap(), country);
                    match country {
                        bili_sub_filter::mods::check::Country::China => {
                            cn_nodes["proxies"]
                                .as_sequence_mut()
                                .unwrap()
                                .push(connect_rst.0.clone());
                        }
                        bili_sub_filter::mods::check::Country::Taiwan => {
                            tw_nodes["proxies"]
                                .as_sequence_mut()
                                .unwrap()
                                .push(connect_rst.0.clone());
                        }
                        bili_sub_filter::mods::check::Country::Hongkang => {
                            hk_nodes["proxies"]
                                .as_sequence_mut()
                                .unwrap()
                                .push(connect_rst.0.clone());
                        }
                        bili_sub_filter::mods::check::Country::Thailand => {
                            th_nodes["proxies"]
                                .as_sequence_mut()
                                .unwrap()
                                .push(connect_rst.0.clone());
                        }
                        bili_sub_filter::mods::check::Country::Singapore => {
                            sg_nodes["proxies"]
                                .as_sequence_mut()
                                .unwrap()
                                .push(connect_rst.0.clone());
                        }
                        bili_sub_filter::mods::check::Country::Mongolia => {
                            mn_nodes["proxies"]
                                .as_sequence_mut()
                                .unwrap()
                                .push(connect_rst.0.clone());
                        }
                        bili_sub_filter::mods::check::Country::Unknown => (),
                    }
                }
            }
            // if let Some(countrys) = check_bili_area(&proxy_node).await {
                
            // }
            
            if test_delay {
                // if let Ok(delay_map) = DELAY_RST_RECEIVER.recv().await {
                if let Some(delay_map) = get_nodes_delay().await {
                    let retain_fn = |a: &serde_yaml::Value| {
                        let delay = delay_map.get(a["name"].as_str().unwrap()).unwrap_or(&0);
                        if *delay != 0 && *delay < 1000 {
                            true
                        } else {
                            false
                        }
                    };
                    cn_nodes["proxies"]
                        .as_sequence_mut()
                        .unwrap()
                        .retain(retain_fn);
                    tw_nodes["proxies"]
                        .as_sequence_mut()
                        .unwrap()
                        .retain(retain_fn);
                    hk_nodes["proxies"]
                        .as_sequence_mut()
                        .unwrap()
                        .retain(retain_fn);
                    th_nodes["proxies"]
                        .as_sequence_mut()
                        .unwrap()
                        .retain(retain_fn);
                    sg_nodes["proxies"]
                        .as_sequence_mut()
                        .unwrap()
                        .retain(retain_fn);
                    mn_nodes["proxies"]
                        .as_sequence_mut()
                        .unwrap()
                        .retain(retain_fn);
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

    // 用 /group/delay/delay?url=http://www.gstatic.com/generate_202&timeout=2000 更方便,为什么我作死要获取单个的 ()

    // let check_delay_fn = tokio::spawn(async move {
    //     loop {println!("-----147");
    //         let channel_code = if let Ok(value) = DELAY_RECEIVER.recv().await {
    //             println!("-----149");
    //             value
    //         } else {
    //             println!("-----152");
    //             continue;
    //         };
    //         match channel_code {
    //             1 => {
    //                 println!("-----157");
    //                 let delay_map: HashMap<serde_yaml::Value, bool> = HashMap::new();
    //                 let mutex_delay_map = Arc::new(Mutex::new(delay_map));
    //                 match get_delay_nodes().await {
    //                     Some(nodes) => {
    //                         println!("-----162");
    //                         let num_of_nodes = nodes.len();
    //                         let mutex_nodes = Arc::new(Mutex::new(nodes));
    //                         let mut handles = Vec::with_capacity(num_of_nodes);
    //                         for _i in 0..10 {
    //                             let new_mutex_delay_map = mutex_delay_map.clone();
    //                             let new_mutex_nodes = mutex_nodes.clone();
    //                             let check = tokio::spawn(async move {
    //                                 let node = {
    //                                     if let Some(value) = new_mutex_nodes.lock().unwrap().pop() {
    //                                         value
    //                                     }else{
    //                                         return;
    //                                     }
    //                                 };
    //                                 let delay = get_node_delay(node["name"].as_str().unwrap(),"JCasbciSCBAISw").await;
    //                                 let ok = {
    //                                     println!("[Debug] delay: {}", delay);
    //                                     if delay == 0 || delay > 1000 {
    //                                         false
    //                                     }else{
    //                                         true
    //                                     }
    //                                 };
    //                                 {
    //                                     // println!("{}", node);
    //                                     new_mutex_delay_map.lock().unwrap().insert(serde_yaml::Value::String(node["name"].as_str().unwrap().to_string()),ok);
    //                                 }
    //                             });
    //                             handles.push(check);
    //                         }
    //                         println!("-----190");
    //                         for handle in handles {
    //                             println!("-----192");
    //                             handle.await.unwrap_or_default();
    //                         }
    //                         let delay_map = (*mutex_delay_map.lock().unwrap()).clone();
    //                         println!("[Debug] delay fi");
    //                         DELAY_RST_SENDER.send(Arc::new(delay_map)).await.unwrap();
    //                         println!("-----198");
    //                     },
    //                     None => {
    //                         println!("-----201");
    //                         DELAY_RST_SENDER.send(Arc::new(HashMap::new())).await.unwrap();
    //                     }
    //                 }
    //             }
    //             _ => {
    //                 panic!("unknown channel code")
    //             }
    //         }
    //     }
    // });

    join!(clash, check_connectivity_fn).1.unwrap_or_default();
}
