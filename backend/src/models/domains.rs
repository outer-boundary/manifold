use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Domain Types

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub banner_url: Option<String>,
    pub icon_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewDomain {
    pub name: String,
    pub details: Option<UpdateDomain>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDomain {
    pub description: Option<String>,
    pub banner_url: Option<String>,
    pub icon_url: Option<String>,
}

// Domain Membership Types

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DomainMembership {
    pub domain_id: Uuid,
    pub user_id: Uuid,
    pub role_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewDomainMembership {
    pub domain_id: Uuid,
    pub user_id: Uuid,
    pub role_name: String,
}