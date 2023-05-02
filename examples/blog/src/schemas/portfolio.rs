use crate::{
    schemas::{
        utc_datetime,
        optional_utc_datetime,
    }
};

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
    #[derive(Serialize)]
    pub struct EditPorfolioRequest {
        pub id: u64,
    }
);

PortfolioBase!(
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Porfolio{
        pub id: u64,
        #[serde(with = "utc_datetime")]
        pub created_at: DateTime<Utc>,
        #[serde(with = "optional_utc_datetime")]
        pub updated_at: Option<DateTime<Utc>>,
    }
);

