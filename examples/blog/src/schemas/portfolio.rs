use crate::{
    schemas::{
        utc_datetime,
        optional_utc_datetime,
    }
};

use uuid::Uuid;

use serde::{Deserialize, Serialize};
use born::{
    public_struct,
    nested_macro,
};

use chrono::{
    DateTime, 
    Utc
};

use url::{
    // Use as_str method again to read entire url again
    Url,
};

public_struct!(
    pub struct PortfolioBase {
        pub title: String,
        pub link: Option<Url>,
        pub description: String,
    }
);

PortfolioBase!(
    #[derive(Deserialize)]
    pub struct CreatePorfolioRequest
);

PortfolioBase!(
    #[derive(Deserialize)]
    pub struct EditPorfolioRequest {
        // pub id: u64,
        pub id: Uuid,
    }
);

PortfolioBase!(
    #[derive(Debug, Serialize, Clone)]
    pub struct PorfolioInDB{
        // pub id: u64,
        pub id: Uuid,
        #[serde(with = "utc_datetime")]
        pub created_at: DateTime<Utc>,
        #[serde(with = "optional_utc_datetime")]
        pub updated_at: Option<DateTime<Utc>>,
    }
);

