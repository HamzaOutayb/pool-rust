use std::collections::HashMap;
use chrono::{DateTime, Utc};
use json::JsonValue;
use chrono::Datelike;

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut res = HashMap::new(); 
    for member in data.members() {
        if let Some(date_str) = member["commit"]["author"]["date"].as_str() {
            if let Ok(d) = date_str.parse::<DateTime<Utc>>() {
                let week = d.iso_week();
                *res.entry(format!("{:?}", week)).or_insert(0) += 1;
            }
        }
    }
    res
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut res = HashMap::new();
    for member in data.members() {
        if let Some(author_name) = member["commit"]["author"]["name"].as_str() {
            *res.entry(author_name.to_string()).or_insert(0) += 1;
        }
    }
    res
}
