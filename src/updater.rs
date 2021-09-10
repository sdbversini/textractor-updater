use std::ffi::OsStr;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use reqwest::header::ACCEPT;

pub struct Updater {
    current_tag: String,
    base_url: String,
    latest_tag: String,
}


#[derive(PartialEq, Debug)]
enum Version {
    UpToDate(),
    Downloaded(),
}

impl Updater {
    pub fn new() -> Updater {
        Updater {
            current_tag: String::from("v0.0.0"),
            latest_tag: String::from("v0.0.0"),
            base_url: String::from("https://github.com/Artikash/Textractor/releases"),
        }
    }
    /// Returns the latest version in the git remote
    fn set_remote_version(&mut self) {
        let latest_url = format!("{}/latest", self.base_url);
        let client = reqwest::blocking::Client::new();
        let resp = client.get(latest_url).header(ACCEPT, "application/json").send().unwrap();
        let json = resp.json::<serde_json::Value>().unwrap();
        self.latest_tag = json.get("tag_name").unwrap().to_string().trim_matches('"').to_owned();
    }
    /// Downloads the file at the specified URL, naming it latest.zip
    fn download_from_url(&mut self, url: &str) {
        let resp = reqwest::blocking::get(url).unwrap();
        let mut file = std::fs::File::create("latest.zip").unwrap();
        let mut contents = Cursor::new(resp.bytes().unwrap());
        std::io::copy(&mut contents, &mut file).unwrap();
    }

    /// Returns the download URL of the latest remote version
    fn get_download_url(&self) -> String {
        let mut version = String::from(&self.latest_tag);
        version.replace_range(0..1, "");
        let url = format!("{}/download/{}/Textractor-{}-Zip-Version-English-Only.zip", self.base_url, self.latest_tag, version);
        String::from(&url)
    }

    /// Checks if up to date and downloads the latest version if not
    fn download_latest(&mut self) -> Version {
        if self.current_tag != self.latest_tag {
            let download_url = self.get_download_url();
            self.download_from_url(&download_url);
            Version::Downloaded()
        } else {
            Version::UpToDate()
        }
    }

    fn extract_archive(&self) {
        let filename = std::path::Path::new("latest.zip");
        let file = std::fs::File::open(&filename).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        let truncate_textractor = |path: &Path| -> PathBuf {
            let components = path.iter().filter(|x| *x != OsStr::new("Textractor"));
            components.collect::<PathBuf>()
        };

        for i in 1..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => truncate_textractor(path),
                None => continue,
            };

            if (&*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = std::fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }
    fn delete_zip() {
        let mut path = PathBuf::from(std::env::current_dir().unwrap());
        path.push("latest.zip");
        match std::fs::remove_file(path){
            Ok(_) => { println!("latest.zip successfully deleted!") }
            Err(e) => { println!("{}", e); }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const URL501: &str = "https://github.com/Artikash/Textractor/releases/download/v5.0.1/Textractor-5.0.1-Zip-Version-English-Only.zip";

    // TODO Will break in new version
    #[test]
    fn test_get_remote_version() {
        let mut updater = Updater::new();
        updater.set_remote_version();
        assert_eq!(updater.latest_tag, "v5.0.1");
    }

    #[test]
    fn test_download_from_url() {
        let mut updater = Updater::new();
        updater.download_from_url(URL501);
    }

    #[test]
    fn test_get_dl_url() {
        let mut updater = Updater::new();
        updater.latest_tag = String::from("v5.0.1");
        assert_eq!(updater.get_download_url(), URL501);
    }

    #[test]
    fn test_download_latest_up_to_date() {
        let mut updater = Updater::new();
        updater.current_tag = String::from("v5.0.1");
        updater.latest_tag = String::from("v5.0.1");
        assert_eq!(updater.download_latest(), Version::UpToDate());
    }

    #[test]
    fn test_download_latest_not_up_to_date() {
        let mut updater = Updater::new();
        updater.current_tag = String::from("v0.0.1");
        assert_eq!(updater.download_latest(), Version::Downloaded());
    }

    #[test]
    fn test_extract_archive() {
        let mut updater = Updater::new();
        updater.download_from_url(URL501);
        updater.extract_archive();
    }


    #[test]
    fn test_remove_zip() {
        Updater::delete_zip();
    }
}