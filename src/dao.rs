
use crate::entities::{AddResultModel, DeleteResultModel, VerificationResultModel};
use crate::prelude::*;

pub async fn add_user_images(bytes: &String) -> impl Responder {
    AddResultModel {}
}

pub async fn verify_user(bytes: &String) -> impl Responder {
    VerificationResultModel { user_id: Some(format!("user_1")) }
}

pub async fn delete_user(bytes: &String) -> impl Responder {
    DeleteResultModel {}
}