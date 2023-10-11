use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewDomain {
    pub display_name: String,
    pub description_text: Option<String>,
    
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,

    pub public: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub id: Uuid,

    pub display_name: String,
    pub description_text: Option<String>,
    
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,

    pub public: bool,

    pub created_at: NaiveDateTime,

}

pub struct DomainWithMemberships (Domain, Vec<DomainMembership>);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DomainMembership {
    pub domain_id: Uuid,
    pub user_id: Uuid,
    pub role_name: String,
}