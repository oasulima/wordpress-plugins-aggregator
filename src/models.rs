// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "info")]
    pub info: Info,

    #[serde(rename = "plugins")]
    pub plugins: Vec<Plugin>,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "page")]
    pub page: i64,

    #[serde(rename = "pages")]
    pub pages: i64,

    #[serde(rename = "results")]
    pub results: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Plugin {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "slug")]
    pub slug: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "author_profile")]
    pub author_profile: String,

    #[serde(rename = "requires")]
    pub requires: String,

    #[serde(rename = "tested")]
    pub tested: Tested,

    #[serde(rename = "requires_php")]
    pub requires_php: RequiresPhp,

    #[serde(rename = "requires_plugins")]
    pub requires_plugins: Vec<String>,

    #[serde(rename = "rating")]
    pub rating: i64,

    #[serde(rename = "ratings")]
    pub ratings: HashMap<String, i64>,

    #[serde(rename = "num_ratings")]
    pub num_ratings: i64,

    #[serde(rename = "support_threads")]
    pub support_threads: i64,

    #[serde(rename = "support_threads_resolved")]
    pub support_threads_resolved: i64,

    #[serde(rename = "active_installs")]
    pub active_installs: i64,

    #[serde(rename = "downloaded")]
    pub downloaded: i64,

    #[serde(rename = "last_updated")]
    pub last_updated: String,

    #[serde(rename = "added")]
    pub added: String,

    #[serde(rename = "homepage")]
    pub homepage: String,

    #[serde(rename = "short_description")]
    pub short_description: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "download_link")]
    pub download_link: String,

    #[serde(rename = "tags")]
    pub tags: HashMap<String, String>,

    #[serde(rename = "donate_link")]
    pub donate_link: String,

    #[serde(rename = "icons")]
    pub icons: Icons,
}

#[derive(Serialize, Deserialize)]
pub struct Icons {
    #[serde(rename = "1x")]
    pub the_1_x: Option<String>,

    #[serde(rename = "2x")]
    pub the_2_x: Option<String>,

    #[serde(rename = "svg")]
    pub svg: Option<String>,

    #[serde(rename = "default")]
    pub icons_default: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequiresPhp {
    Bool(bool),

    String(String),
}

#[derive(Serialize, Deserialize)]
pub enum Tested {
    #[serde(rename = "6.5.5")]
    The655,

    #[serde(rename = "6.6.2")]
    The662,

    #[serde(rename = "6.7")]
    The67,
}