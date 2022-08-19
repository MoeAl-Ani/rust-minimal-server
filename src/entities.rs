use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::prelude::{Reply, ReplyItem, Responder, Response};

#[derive(Debug, Clone, Serialize)]
pub struct AddResultModel {}

#[derive(Debug, Clone, Serialize)]
pub struct VerificationResultModel {
    pub(crate) user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteResultModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessageModel {
    message: String,
}


#[async_trait]
impl Responder for AddResultModel {
    type Item = Response;

    async fn transform(self) -> Self::Item {
        let body = serde_json::to_string(&self).unwrap();
        Response { status: 200, body: Box::new(body) }
    }
}


#[async_trait]
impl Responder for DeleteResultModel {
    type Item = Response;

    async fn transform(self) -> Self::Item {
        let body = serde_json::to_string(&self).unwrap();
        Response { status: 204, body: Box::new(body) }
    }
}


#[async_trait]
impl Responder for VerificationResultModel {
    type Item = Response;

    async fn transform(self) -> Self::Item {
        let body = serde_json::to_string(&self).unwrap();
        Response { status: 200, body: Box::new(body) }
    }
}