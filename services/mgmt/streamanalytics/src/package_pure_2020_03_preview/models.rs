#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename = "createdDate", skip_serializing)]
    pub created_date: Option<String>,
    #[serde(rename = "clusterId", skip_serializing)]
    pub cluster_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ClusterProvisioningState>,
    #[serde(rename = "capacityAllocated", skip_serializing)]
    pub capacity_allocated: Option<i32>,
    #[serde(rename = "capacityAssigned", skip_serializing)]
    pub capacity_assigned: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Cluster>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterJob {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "streamingUnits", skip_serializing)]
    pub streaming_units: Option<i32>,
    #[serde(rename = "jobState", skip_serializing)]
    pub job_state: Option<JobState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterJobListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ClusterJob>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cluster_sku::Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod cluster_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Default,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClusterProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    InProgress,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobState {
    Created,
    Starting,
    Running,
    Stopping,
    Stopped,
    Deleting,
    Failed,
    Degraded,
    Restarting,
    Scaling,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointProperties>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointProperties {
    #[serde(rename = "createdDate", skip_serializing)]
    pub created_date: Option<String>,
    #[serde(rename = "manualPrivateLinkServiceConnections", skip_serializing_if = "Vec::is_empty")]
    pub manual_private_link_service_connections: Vec<PrivateLinkServiceConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkServiceConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionProperties {
    #[serde(rename = "privateLinkServiceId", skip_serializing_if = "Option::is_none")]
    pub private_link_service_id: Option<String>,
    #[serde(rename = "groupIds", skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
    #[serde(rename = "requestMessage", skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[serde(rename = "privateLinkServiceConnectionState", skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkConnectionState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkConnectionState {
    #[serde(skip_serializing)]
    pub status: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", skip_serializing)]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointListResult {
    #[serde(skip_serializing)]
    pub value: Vec<PrivateEndpoint>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<error::Error>,
}
pub mod error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorDetails>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
