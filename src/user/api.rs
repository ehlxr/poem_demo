use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi};

use slab::Slab;
use tokio::sync::Mutex;
use tracing::info;

use crate::CACHE;

use super::*;

#[derive(Default)]
pub(crate) struct Api {
    users: Mutex<Slab<User>>,
}

#[OpenApi]
impl Api {
    /// Create a new user
    #[oai(path = "/users", method = "post", tag = "ApiTags::User")]
    async fn create_user(&self, user: Json<User>, data: Data<&Token>) -> CreateUserResponse {
        let Token(token) = data.0;
        assert_eq!(token, "token123");

        let m_guard = CACHE.read().await;
        let rt = m_guard.get(&"token".to_string()).unwrap();
        info!("refresh_token is {}", rt);

        let mut users = self.users.lock().await;
        let id = users.insert(user.0) as i64;
        CreateUserResponse::Ok(Json(id))
    }

    /// Find user by id
    #[oai(path = "/users/:user_id", method = "get", tag = "ApiTags::User")]
    async fn find_user(&self, user_id: Path<i64>) -> FindUserResponse {
        let users = self.users.lock().await;
        match users.get(user_id.0 as usize) {
            Some(user) => FindUserResponse::Ok(Json(user.clone())),
            None => FindUserResponse::NotFound,
        }
    }

    /// Delete user by id
    #[oai(path = "/users/:user_id", method = "delete", tag = "ApiTags::User")]
    async fn delete_user(&self, user_id: Path<i64>) -> DeleteUserResponse {
        let mut users = self.users.lock().await;
        let user_id = user_id.0 as usize;
        if users.contains(user_id) {
            users.remove(user_id);
            DeleteUserResponse::Ok
        } else {
            DeleteUserResponse::NotFound
        }
    }

    /// Update user by id
    #[oai(path = "/users/:user_id", method = "put", tag = "ApiTags::User")]
    async fn put_user(&self, user_id: Path<i64>, update: Json<UpdateUser>) -> UpdateUserResponse {
        let mut users = self.users.lock().await;
        match users.get_mut(user_id.0 as usize) {
            Some(user) => {
                if let Some(name) = update.0.name {
                    user.name = name;
                }
                if let Some(password) = update.0.password {
                    user.password = password;
                }
                UpdateUserResponse::Ok
            }
            None => UpdateUserResponse::NotFound,
        }
    }
}
