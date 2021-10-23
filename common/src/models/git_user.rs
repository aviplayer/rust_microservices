use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;
#[derive(Serialize, Validate, Deserialize, Debug, Clone, PartialEq)]
pub struct User{
    pub login: String,
    pub id: i32,
    pub node_id: String,
    #[validate(url)]  // some pretty dumb restriction
    pub avatar_url: String,
    pub site_admin: bool
}
