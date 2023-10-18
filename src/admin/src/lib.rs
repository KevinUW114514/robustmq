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

use axum::{routing::get, Router};
use rlog;
use std::{net::SocketAddr, str::FromStr};
use tokio::runtime::Runtime;

const ROUTE_ROOT: &str = "/";
const ROUTE_METRICS: &str = "/metrics";
const ROUTE_MANAGEMENT_API_OVERVIEW: &str = "/api/overview";
const ROUTE_MANAGEMENT_API_CLUSTER: &str = "/api/cluster-name";
const ROUTE_MANAGEMENT_API_NODES: &str = "/api/nodes";
const ROUTE_MANAGEMENT_API_NODE_NAME: &str = "/api/nodes/name";

mod management_api;
mod prometheus;
mod welcome;


fn router_construct() -> Router {
    // define Management API routes separately
    let management_api_routes = Router::new()
        .route(
            ROUTE_MANAGEMENT_API_OVERVIEW,
            get(management_api::api_overview_get_handler),
        )
        .route(
            ROUTE_MANAGEMENT_API_CLUSTER,
            get(management_api::api_cluster_get_handler),
        )
        .route(
            ROUTE_MANAGEMENT_API_CLUSTER,
            post(management_api::api_cluster_post_handler),
        )
        .route(
            ROUTE_MANAGEMENT_API_NODES,
            get(management_api::api_nodes_handler),
        )
        .route(
            ROUTE_MANAGEMENT_API_NODE_NAME,
            get(management_api::api_node_name_handler),
        );

    let other_routes = Router::new()
        .route(ROUTE_METRICS, get(prometheus::handler))
        .route(ROUTE_ROOT, get(welcome::handler));

    let router = Router::new()
        .merge(management_api_routes)
        .merge(other_routes);
    router


pub fn start(addr: String, port: Option<u16>, worker_threads: usize) -> Runtime {
    let runtime: Runtime = tokio::runtime::Builder::new_current_thread()
        .worker_threads(worker_threads)
        .max_blocking_threads(2048)
        .thread_name("admin-http")
        .enable_io()
        .build()
        .unwrap();
    let _guard = runtime.enter();

    runtime.block_on(async move {
        let app = router_construct();

        let ip = format!("{}:{}", addr, port.unwrap());
        let ip_addr = SocketAddr::from_str(&ip).unwrap();

        rlog::info(&format!("http server start success. bind:{}", ip));

        axum::Server::bind(&ip_addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    return runtime;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}