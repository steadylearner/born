use serde::{
    Deserialize, 
    Serialize, 
};

// https://github.com/chronotope/chrono-tz
use chrono::{
    DateTime, 
    Utc
};
use url::{
    // Use as_str method again to read entire url again
    Url,
};

// Visit this at the browser to see how the datas are first
// https://dev.to/api/articles/me/published

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DevToBlog {
    pub id: u64,
    pub cover_image: Option<Url>,
    pub url: Url,
    pub title: String,
    pub slug: String,
    // path: String,
    pub description: String,
    pub tag_list: Vec<String>,
    pub comments_count: u16,
    pub public_reactions_count: u16,
    
    // pub reading_time_minutes: u8,
    pub body_markdown: String,
    pub reading_time_minutes: u8,

    #[serde(with = "utc_datetime")]
    pub published_timestamp: DateTime<Utc>,
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