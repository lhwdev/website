use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, get};

#[get("/<file..>")]
pub async fn get_static(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static/").join(file);
    match NamedFile::open(&path).await {
        Ok(file) => Some(file),
        _ => match NamedFile::open(&path.join("index.html")).await {
            Ok(file) => Some(file),
            _ => None,
        },
    }
}
