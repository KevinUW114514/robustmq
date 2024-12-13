use mockall::{automock, mock};
use mockall::predicate::*;

use protocol::broker_mqtt::broker_mqtt_admin::{
    ListTopicReply, ListTopicRequest, MqttTopic as MqttTopicProto,
};
use metadata_struct::mqtt::topic::MqttTopic;
use dashmap::DashMap;
use super::super::services::GrpcAdminServices;

use std::sync::Arc;

struct ClientPool {}
#[automock]
impl ClientPool {}

struct ConnectionManager {}
#[automock]
impl ConnectionManager {}

struct MockCacheManager {
  topic_info: DashMap<String, MqttTopic>,
}

impl MockCacheManager {
    pub fn new(topic_map: DashMap<String, MqttTopic>) -> Self {
        MockCacheManager { topic_info: topic_map }
    }

    pub fn get_topic_by_name(&self, topic_name: &str) -> Option<MqttTopic> {
        if let Some(topic) = self.topic_info.get(topic_name) {
          return Some(topic.clone());
      }
      None
    }
}

#[tokio::test]
async fn test_mqtt_broker_list_topic() {
  let mocked_client_pool = MockClientPool::new();
  let mocked_connection_manager = MockConnectionManager::new();
  
  let mocked_mqtt_topic_data = vec![
    MqttTopic {
      topic_id: String::from("topic-123"),
      cluster_name: String::from("cluster-xyz"),
      topic_name: String::from("example_topic"),
      retain_message: Some(vec![1, 2, 3, 4, 5]),
      retain_message_expired_at: Some(1700000000),
    },
    MqttTopic {
      topic_id: String::from("topic-456"),
      cluster_name: String::from("cluster-0"),
      topic_name: String::from("example_topic_2"),
      retain_message: Some(vec![6, 7, 8, 9, 10]),
      retain_message_expired_at: Some(1700000000),
    },
    MqttTopic {
      topic_id: String::from("topic-456"),
      cluster_name: String::from("cluster-1"),
      topic_name: String::from("example_topic_6"),
      retain_message: None,
      retain_message_expired_at: None,
    },
    MqttTopic {
      topic_id: String::from("topic-456"),
      cluster_name: String::from("cluster-2"),
      topic_name: String::from("example_topic_66"),
      retain_message: None,
      retain_message_expired_at: None,
    },
  ];

  let mocked_topic_map: DashMap<String, MqttTopic> = DashMap::new();
  for topic in mocked_mqtt_topic_data {
      mocked_topic_map.insert(topic.topic_name.clone(), topic);
  }

  let mut mocked_cache_manager = MockCacheManager::new(mocked_topic_map);

  let mut grpc_admin_services = GrpcAdminServices::new(
    Arc::new(mocked_client_pool), 
    Arc::new(mocked_cache_manager), 
    Arc::new(mocked_connection_manager)
  );

}

