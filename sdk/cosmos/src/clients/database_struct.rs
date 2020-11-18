use crate::clients::{CollectionStruct, CosmosClient2, UserStruct};
use crate::params::database::*;
use crate::responses::*;
use crate::traits::*;
use crate::{requests, CosmosClient};
use crate::{ConsistencyLevelOption, ResourceType};
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use azure_core::No;
use std::borrow::Cow;

/// A builder for database-centric Cosmos requests, created by
/// [`CosmosClient::database`](CosmosClient::database)
pub struct DatabaseClient2 {
    client: CosmosClient2,
    db_name: String,
}

impl DatabaseClient2 {
    pub async fn list_collections(&self) {
        // requests::ListCollectionsBuilder::new(self)
    }

    pub async fn get_database(&self) {
        // requests::GetDatabaseBuilder::new(self)
    }

    pub async fn delete_database(&self) {
        // requests::DeleteDatabaseBuilder::new(self)
    }

    pub async fn create_database(
        &self,
        params: CreateDatabaseParams,
    ) -> Result<CreateDatabaseResponse, AzureError> {
        #[derive(Serialize, Debug)]
        struct CreateDatabaseRequest<'a> {
            pub id: &'a str,
        }

        let req = serde_json::to_string(&CreateDatabaseRequest { id: &self.db_name })?;

        let request =
            self.client
                .prepare_request("dbs", hyper::Method::POST, ResourceType::Databases);

        let request = UserAgentOption::add_header(&params, request);
        let request = ActivityIdOption::add_header(&params, request);
        let request = ConsistencyLevelOption::add_header(params.consistency_level, request);

        // TODO: This should probably be put in a function at the CosmosClient level

        let request = request.body(hyper::Body::from(req))?; // todo: set content-length here and elsewhere without builders

        debug!("create database request prepared == {:?}", request);

        let future_response = self.client.client.request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }

    pub async fn list_users(&self) {
        // requests::ListUsersBuilder::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    cosmos_client: Cow<'a, C>,
    database_name: Cow<'a, str>,
}

impl<'a, C> DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    pub(crate) fn new(cosmos_client: Cow<'a, C>, database_name: Cow<'a, str>) -> Self {
        DatabaseStruct {
            cosmos_client,
            database_name,
        }
    }
}

impl<'a, C> HasHyperClient for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }
}

impl<'a, C> HasCosmosClient<C> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        &self.cosmos_client
    }
}

impl<'a, C> DatabaseClient<C> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    fn database_name(&self) -> &str {
        &self.database_name
    }

    fn list_collections(&self) -> requests::ListCollectionsBuilder<'_, C> {
        requests::ListCollectionsBuilder::new(self)
    }

    fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_, C> {
        requests::GetDatabaseBuilder::new(self)
    }

    fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, C> {
        requests::DeleteDatabaseBuilder::new(self)
    }

    fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, C, No, No, No, No> {
        requests::CreateCollectionBuilder::new(self)
    }

    fn list_users(&self) -> requests::ListUsersBuilder<'_, '_, C> {
        requests::ListUsersBuilder::new(self)
    }
}

impl<'a, C> WithCollectionClient<'a, C, Self, CollectionStruct<'a, C, Self>>
    for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn with_collection_client<IntoCowStr>(
        &'a self,
        collection_name: IntoCowStr,
    ) -> CollectionStruct<'a, C, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        CollectionStruct::new(Cow::Borrowed(self), collection_name.into())
    }
}

impl<'a, C> IntoCollectionClient<'a, C, Self, CollectionStruct<'a, C, Self>>
    for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn into_collection_client<IntoCowStr>(
        self,
        collection_name: IntoCowStr,
    ) -> CollectionStruct<'a, C, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        CollectionStruct::new(Cow::Owned(self), collection_name.into())
    }
}

impl<'a, C> WithUserClient<'a, C, Self, UserStruct<'a, C, Self>> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn with_user_client<IntoCowStr>(&'a self, user_name: IntoCowStr) -> UserStruct<'a, C, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        UserStruct::new(Cow::Borrowed(self), user_name.into())
    }
}

impl<'a, C> IntoUserClient<'a, C, Self, UserStruct<'a, C, Self>> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn into_user_client<IntoCowStr>(self, user_name: IntoCowStr) -> UserStruct<'a, C, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        UserStruct::new(Cow::Owned(self), user_name.into())
    }
}
