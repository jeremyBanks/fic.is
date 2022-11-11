use std::sync::Arc;

use thiserror::Error;

use crate::blob::Blip;
use crate::blob::Blob;
use crate::generic::never;
use crate::storage::sqlite::SqliteStorage;
use crate::AnyRequest;
use crate::AnyResponse;

#[derive(Debug, Default)]
pub struct Context<Request: crate::Request> {
    storage: Option<Arc<SqliteStorage>>,

    request_and_aliases: Vec<Request>,
}

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum ContextError {}

impl<Request: crate::Request> Context<Request> {
    pub fn new(storage: impl Into<Option<Arc<SqliteStorage>>>) -> Self {
        let storage = storage.into();
        Context {
            storage,
            ..Default::default()
        }
    }

    pub fn query(&mut self, request: AnyRequest) -> Result<AnyResponse, never> {
        todo!()
    }

    pub fn get_blob<Rep>(&self, id: impl Into<Blip<Rep>>) -> Result<Option<Blob<Rep>>, never> {
        todo!()
    }

    pub fn insert_blob<Rep>(&self, data: impl Into<Blob<Rep>>) -> Result<Blip<Rep>, never> {
        todo!()
    }

    pub fn get_responses(&self, request: Request) -> Result<Request::Response, never> {
        todo!()
    }

    pub fn insert_response<OtherRequest: crate::Request>(
        &self,
        request: OtherRequest,
        response: OtherRequest::Response,
    ) {
        todo!()
    }

    /// Adds an alias request that will also be associated with this request's
    /// result.
    pub fn populate(&self, request: Request) {}
}
