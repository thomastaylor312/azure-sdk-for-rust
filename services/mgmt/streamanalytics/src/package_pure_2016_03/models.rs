#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
        #[serde(skip_serializing)]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingJob {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StreamingJobProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingJobProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "jobId", skip_serializing)]
    pub job_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "jobState", skip_serializing)]
    pub job_state: Option<String>,
    #[serde(rename = "outputStartMode", skip_serializing_if = "Option::is_none")]
    pub output_start_mode: Option<OutputStartMode>,
    #[serde(rename = "outputStartTime", skip_serializing_if = "Option::is_none")]
    pub output_start_time: Option<String>,
    #[serde(rename = "lastOutputEventTime", skip_serializing)]
    pub last_output_event_time: Option<String>,
    #[serde(rename = "eventsOutOfOrderPolicy", skip_serializing_if = "Option::is_none")]
    pub events_out_of_order_policy: Option<EventsOutOfOrderPolicy>,
    #[serde(rename = "outputErrorPolicy", skip_serializing_if = "Option::is_none")]
    pub output_error_policy: Option<OutputErrorPolicy>,
    #[serde(rename = "eventsOutOfOrderMaxDelayInSeconds", skip_serializing_if = "Option::is_none")]
    pub events_out_of_order_max_delay_in_seconds: Option<i32>,
    #[serde(rename = "eventsLateArrivalMaxDelayInSeconds", skip_serializing_if = "Option::is_none")]
    pub events_late_arrival_max_delay_in_seconds: Option<i32>,
    #[serde(rename = "dataLocale", skip_serializing_if = "Option::is_none")]
    pub data_locale: Option<String>,
    #[serde(rename = "compatibilityLevel", skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<CompatibilityLevel>,
    #[serde(rename = "createdDate", skip_serializing)]
    pub created_date: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<Transformation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<Output>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<Function>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingJobListResult {
    #[serde(skip_serializing)]
    pub value: Vec<StreamingJob>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartStreamingJobParameters {
    #[serde(rename = "outputStartMode", skip_serializing_if = "Option::is_none")]
    pub output_start_mode: Option<OutputStartMode>,
    #[serde(rename = "outputStartTime", skip_serializing_if = "Option::is_none")]
    pub output_start_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputStartMode {
    JobStartTime,
    CustomTime,
    LastOutputEventTime,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventsOutOfOrderPolicy {
    Adjust,
    Drop,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputErrorPolicy {
    Stop,
    Drop,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    #[serde(rename = "1.0")]
    _1_0,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<InputProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputProperties {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization: Option<Serialization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamInputProperties {
    #[serde(flatten)]
    pub input_properties: InputProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource: Option<StreamInputDataSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceInputProperties {
    #[serde(flatten)]
    pub input_properties: InputProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource: Option<ReferenceInputDataSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamInputDataSource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobStreamInputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStreamInputDataSourceProperties {
    #[serde(flatten)]
    pub blob_data_source_properties: BlobDataSourceProperties,
    #[serde(rename = "sourcePartitionCount", skip_serializing_if = "Option::is_none")]
    pub source_partition_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubStreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubStreamInputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubStreamInputDataSourceProperties {
    #[serde(flatten)]
    pub event_hub_data_source_properties: EventHubDataSourceProperties,
    #[serde(rename = "consumerGroupName", skip_serializing_if = "Option::is_none")]
    pub consumer_group_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubStreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTHubStreamInputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubStreamInputDataSourceProperties {
    #[serde(rename = "iotHubNamespace", skip_serializing_if = "Option::is_none")]
    pub iot_hub_namespace: Option<String>,
    #[serde(rename = "sharedAccessPolicyName", skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_name: Option<String>,
    #[serde(rename = "sharedAccessPolicyKey", skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_key: Option<String>,
    #[serde(rename = "consumerGroupName", skip_serializing_if = "Option::is_none")]
    pub consumer_group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceInputDataSource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobReferenceInputDataSource {
    #[serde(flatten)]
    pub reference_input_data_source: ReferenceInputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobReferenceInputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobReferenceInputDataSourceProperties {
    #[serde(flatten)]
    pub blob_data_source_properties: BlobDataSourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobDataSourceProperties {
    #[serde(rename = "storageAccounts", skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<StorageAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "pathPattern", skip_serializing_if = "Option::is_none")]
    pub path_pattern: Option<String>,
    #[serde(rename = "dateFormat", skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(rename = "timeFormat", skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(rename = "accountName", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "accountKey", skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusDataSourceProperties {
    #[serde(rename = "serviceBusNamespace", skip_serializing_if = "Option::is_none")]
    pub service_bus_namespace: Option<String>,
    #[serde(rename = "sharedAccessPolicyName", skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_name: Option<String>,
    #[serde(rename = "sharedAccessPolicyKey", skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubDataSourceProperties {
    #[serde(flatten)]
    pub service_bus_data_source_properties: ServiceBusDataSourceProperties,
    #[serde(rename = "eventHubName", skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diagnostics {
    #[serde(skip_serializing)]
    pub conditions: Vec<DiagnosticCondition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticCondition {
    #[serde(skip_serializing)]
    pub since: Option<String>,
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Input>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Serialization {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsvSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CsvSerializationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsvSerializationProperties {
    #[serde(rename = "fieldDelimiter", skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JsonSerializationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonSerializationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<JsonOutputSerializationFormat>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvroSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvroSerializationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvroSerializationProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Encoding {
    #[serde(rename = "UTF8")]
    Utf8,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JsonOutputSerializationFormat {
    LineSeparated,
    Array,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTestStatus {
    #[serde(skip_serializing)]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transformation {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransformationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransformationProperties {
    #[serde(rename = "streamingUnits", skip_serializing_if = "Option::is_none")]
    pub streaming_units: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OutputProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource: Option<OutputDataSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization: Option<Serialization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputDataSource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobOutputDataSourceProperties {
    #[serde(flatten)]
    pub blob_data_source_properties: BlobDataSourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureTableOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureTableOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureTableOutputDataSourceProperties {
    #[serde(rename = "accountName", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "accountKey", skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[serde(rename = "rowKey", skip_serializing_if = "Option::is_none")]
    pub row_key: Option<String>,
    #[serde(rename = "columnsToRemove", skip_serializing_if = "Vec::is_empty")]
    pub columns_to_remove: Vec<String>,
    #[serde(rename = "batchSize", skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubOutputDataSourceProperties {
    #[serde(flatten)]
    pub event_hub_data_source_properties: EventHubDataSourceProperties,
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseOutputDataSourceProperties {
    #[serde(flatten)]
    pub azure_sql_database_data_source_properties: AzureSqlDatabaseDataSourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentDbOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DocumentDbOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentDbOutputDataSourceProperties {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "accountKey", skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "collectionNamePattern", skip_serializing_if = "Option::is_none")]
    pub collection_name_pattern: Option<String>,
    #[serde(rename = "partitionKey", skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusQueueOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueOutputDataSourceProperties {
    #[serde(flatten)]
    pub service_bus_data_source_properties: ServiceBusDataSourceProperties,
    #[serde(rename = "queueName", skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[serde(rename = "propertyColumns", skip_serializing_if = "Vec::is_empty")]
    pub property_columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusTopicOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicOutputDataSourceProperties {
    #[serde(flatten)]
    pub service_bus_data_source_properties: ServiceBusDataSourceProperties,
    #[serde(rename = "topicName", skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[serde(rename = "propertyColumns", skip_serializing_if = "Vec::is_empty")]
    pub property_columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PowerBiOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiOutputDataSourceProperties {
    #[serde(flatten)]
    pub o_auth_based_data_source_properties: OAuthBasedDataSourceProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataLakeStoreOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDataLakeStoreOutputDataSourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataLakeStoreOutputDataSourceProperties {
    #[serde(flatten)]
    pub o_auth_based_data_source_properties: OAuthBasedDataSourceProperties,
    #[serde(rename = "accountName", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "filePathPrefix", skip_serializing_if = "Option::is_none")]
    pub file_path_prefix: Option<String>,
    #[serde(rename = "dateFormat", skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(rename = "timeFormat", skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Output>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseDataSourceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthBasedDataSourceProperties {
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "tokenUserPrincipalName", skip_serializing_if = "Option::is_none")]
    pub token_user_principal_name: Option<String>,
    #[serde(rename = "tokenUserDisplayName", skip_serializing_if = "Option::is_none")]
    pub token_user_display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Function {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FunctionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionProperties {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFunctionProperties {
    #[serde(flatten)]
    pub function_properties: FunctionProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScalarFunctionConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFunctionConfiguration {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<FunctionInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<FunctionOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding: Option<FunctionBinding>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionInput {
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "isConfigurationParameter", skip_serializing_if = "Option::is_none")]
    pub is_configuration_parameter: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionOutput {
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionBinding {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningWebServiceFunctionBinding {
    #[serde(flatten)]
    pub function_binding: FunctionBinding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMachineLearningWebServiceFunctionBindingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningWebServiceFunctionBindingProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<AzureMachineLearningWebServiceInputs>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<AzureMachineLearningWebServiceOutputColumn>,
    #[serde(rename = "batchSize", skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningWebServiceInputs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "columnNames", skip_serializing_if = "Vec::is_empty")]
    pub column_names: Vec<AzureMachineLearningWebServiceInputColumn>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningWebServiceInputColumn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "mapTo", skip_serializing_if = "Option::is_none")]
    pub map_to: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningWebServiceOutputColumn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFunctionBinding {
    #[serde(flatten)]
    pub function_binding: FunctionBinding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JavaScriptFunctionBindingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFunctionBindingProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRetrieveDefaultDefinitionParameters {
    #[serde(rename = "bindingType", skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningWebServiceFunctionRetrieveDefaultDefinitionParameters {
    #[serde(flatten)]
    pub function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters,
    #[serde(rename = "bindingRetrievalProperties", skip_serializing_if = "Option::is_none")]
    pub binding_retrieval_properties: Option<AzureMachineLearningWebServiceFunctionBindingRetrievalProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningWebServiceFunctionBindingRetrievalProperties {
    #[serde(rename = "executeEndpoint", skip_serializing_if = "Option::is_none")]
    pub execute_endpoint: Option<String>,
    #[serde(rename = "udfType", skip_serializing_if = "Option::is_none")]
    pub udf_type: Option<UdfType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFunctionRetrieveDefaultDefinitionParameters {
    #[serde(flatten)]
    pub function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters,
    #[serde(rename = "bindingRetrievalProperties", skip_serializing_if = "Option::is_none")]
    pub binding_retrieval_properties: Option<JavaScriptFunctionBindingRetrievalProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFunctionBindingRetrievalProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "udfType", skip_serializing_if = "Option::is_none")]
    pub udf_type: Option<UdfType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Function>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UdfType {
    Scalar,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionQuotasListResult {
    #[serde(skip_serializing)]
    pub value: Vec<SubscriptionQuota>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionQuota {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing)]
    pub properties: Option<subscription_quota::Properties>,
}
pub mod subscription_quota {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "maxCount", skip_serializing)]
        pub max_count: Option<i32>,
        #[serde(rename = "currentCount", skip_serializing)]
        pub current_count: Option<i32>,
    }
}
