// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::command;
use clap::Parser;
use common_config::{meta::MetaConfig, server::RobustConfig, DEFAULT_META_CONFIG, DEFAULT_SERVER_CONFIG};
use lazy_static::lazy_static;
use common_log::log;
use common_version::version;
use admin;
use common_metrics::server::ServerMetrics;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};
use tokio::{runtime::Runtime, signal};

#[derive(Parser, Debug)]
#[command(author="robustmq", version="1.1", about="RobustMQ: Next generation cloud-native converged high-performance message queue.", long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    /// broker server configuration file path
    #[arg(short, long, default_value_t=String::from(DEFAULT_SERVER_CONFIG))]
    server_conf: String,

    /// MetaService Indicates the path of the configuration file
    #[arg(short, long, default_value_t=String::from(DEFAULT_META_CONFIG))]
    meta_conf: String,
}

lazy_static! {
    static ref SERVER_METRICS: ServerMetrics = ServerMetrics::new();
}

fn main() {
    let args = ArgsParams::parse();
    log::new();

    let server_conf: RobustConfig = common_config::parse_server(&args.server_conf);
    let meta_conf: MetaConfig = common_config::parse_meta(&args.meta_conf);

    SERVER_METRICS.init();

    let admin_server = admin::AdminServer::new(&server_conf);
    let admin_runtime = admin_server.start();
    
    start_broker(&server_conf);
    SERVER_METRICS.set_server_status_running();
    log::server_info("RobustMQ Server was successfully started");
    version::banner();
    shutdown_hook(admin_runtime);
}

fn start_broker(_: &RobustConfig) {}

fn shutdown_hook(runtime: Runtime) {
    let (sx_sender, rx_receiver): (Sender<u16>, Receiver<u16>) = mpsc::channel();

    runtime.spawn(async move {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                log::info("Process receives the signal ctrl + c");
                sx_sender.send(1).unwrap();
            },
            c2 = terminate => {
                sx_sender.send(1).unwrap();
                println!("3333{:?}",c2)
        },
        }
    });

    loop {
        match rx_receiver.recv() {
            Ok(value) => {
                println!("{}", value);
                if value == 3 {
                    runtime.shutdown_timeout(Duration::from_secs(1000));
                    break;
                }
            }
            Err(_) => {}
        }
    }
}