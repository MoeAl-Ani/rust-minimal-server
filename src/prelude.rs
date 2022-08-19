use crate::prelude::ReplyItem::Item;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[async_trait]
pub trait Responder {
    type Item: Into<Reply>;
    async fn transform(self) -> Self::Item;
}

pub struct Reply(pub ReplyItem);

pub enum ReplyItem {
    Item(Response)
}

#[async_trait]
impl Responder for Reply {
    type Item = Reply;
    async fn transform(self) -> Self::Item {
        self
    }
}

#[async_trait]
impl Responder for Response {
    type Item = Reply;

    async fn transform(self) -> Self::Item {
        Reply(ReplyItem::Item(self))
    }
}

impl From<ReplyItem> for Response {
    fn from(reply_item: ReplyItem) -> Self {
        match reply_item {
            Item(response_model) => {
                response_model
            }
        }
    }
}

impl From<Response> for Reply {
    fn from(data: Response) -> Self {
        Reply(ReplyItem::Item(data))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub body: Box<String>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Command {
    ADD,
    DELETE,
    VERIFY,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    pub authorization: String,
    pub body: String,
    pub method: Command,
}