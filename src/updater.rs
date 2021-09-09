use serde::Deserialize;
use reqwest::header::ACCEPT;
use std::io::Cursor;

pub struct Updater<'a> {
    current_tag: &'a str,
    latest_url: &'a str,
}

impl<'a> Updater<'a> {
    pub fn new() -> Updater<'a> {
        Updater {
            current_tag: "v0.0.0",
            latest_url: "https://github.com/Artikash/Textractor/releases/latest",
        }
    }
    fn get_remote_version(&self) -> String {
        let client = reqwest::blocking::Client::new();
        let resp = client.get(self.latest_url).header(ACCEPT, "application/json").send().unwrap();
        let json = resp.json::<serde_json::Value>().unwrap();
        json.get("tag_name").unwrap().to_string().trim_matches('"').to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_remote_version() {
        let updater = Updater::new();
        let tag = updater.get_remote_version();
        assert_eq!(tag, "v5.0.1");
    }
}