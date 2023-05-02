use serde::{
    Deserialize, 
    Serialize, 
};

// https://github.com/chronotope/chrono-tz
use chrono::{
    DateTime, 
    Utc
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DevToArticle {
    id: u64,
    cover_image: Option<String>,
    url: String,
    title: String,
    slug: String,
    // path: String,
    description: String,
    tag_list: Vec<String>,
    comments_count: u16,
    public_reactions_count: u16,
    
    // body_markdown: String,

    // published_timestamp: String,
    #[serde(with = "utc_datetime")]
    published_timestamp: DateTime<Utc>,
}

mod utc_datetime {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = date.format(FORMAT).to_string();
        serializer.serialize_str(&formatted)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}