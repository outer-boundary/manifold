pub struct NewDomain {
    pub display_name: String,
    pub description_text: Option<String>,
    
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
}

pub struct Domain {
    pub display_name: String,
    pub description_text: Option<String>,
    
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,

    pub memberships: Vec<DomainMembership>,

    pub created_at: NaiveDateTime,
}

pub struct DomainMembership {
    user_id: String,
    role: String,
}