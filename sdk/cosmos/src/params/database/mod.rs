use crate::prelude::*;
use azure_core::prelude::*;

#[derive(Default, Debug)]
pub struct CreateDatabaseParams {
    pub(crate) user_agent: Option<String>,
    pub(crate) activity_id: Option<String>,
    pub(crate) consistency_level: Option<ConsistencyLevel>,
}

// Temporary until we create a better way to handle these options. As mentioned in `lib.rs`, a
// `HeaderOption` trait that each custom type can implement might be better

impl<'a> UserAgentOption<'a> for CreateDatabaseParams {
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent.as_ref()
    }
}

impl<'a> ActivityIdOption<'a> for CreateDatabaseParams {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id.as_ref()
    }
}
