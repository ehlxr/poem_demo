pub(crate) mod api;

use poem_openapi::{
    payload::Json,
    types::{Email, Password},
    ApiResponse, Object, Tags,
};

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

/// Create user schema
#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct User {
    /// Id
    #[oai(read_only)]
    id: i64,
    /// Name
    #[oai(validator(max_length = 64))]
    name: String,
    /// Password
    #[oai(validator(max_length = 32))]
    password: Password,
    email: Email,
}

/// Update user schema
#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UpdateUser {
    /// Name
    name: Option<String>,
    /// Password
    password: Option<Password>,
}

#[derive(ApiResponse)]
enum CreateUserResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<i64>),
}

#[derive(ApiResponse)]
enum FindUserResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<User>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum DeleteUserResponse {
    /// Returns when the user is successfully deleted.
    #[oai(status = 200)]
    Ok,
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum UpdateUserResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok,
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(Clone)]
pub(crate) struct Token(pub(crate) String);
