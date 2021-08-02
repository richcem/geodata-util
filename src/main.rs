mod error;

use bytes::Bytes;
use crate::error::{AppError, AppResult};
use std::{env, fs};

const GEOIP_DATA_URL: &str =
    "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/geoip.dat";
const GEOSITE_DATA_URL: &str =
    "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/geosite.dat";

fn main() -> AppResult<()> {

    env_logger::init();

    let workdir = env::current_dir()?;
    let file_geoip = workdir.join("geoip.dat");
    let file_geosite = workdir.join("geosite.dat");

    // 更新geoip.dat
    let bytes = download_from_url(GEOIP_DATA_URL)?;
    fs::write(file_geoip, bytes)?;
    println!("update geoip.dat success");

    let bytes = download_from_url(GEOSITE_DATA_URL)?;
    fs::write(file_geosite, bytes)?;
    println!("update geosite.dat success");

    Ok(())
}

// download from url return bytes
fn download_from_url(url: &str) -> AppResult<Bytes> {

    let resp = reqwest::blocking::get(url)?;

    // debug!("Request from {}", url);
    // debug!("{:#?}", resp);

    if !resp.status().is_success() {
        return Err(AppError::new(format!("http code: {}", resp.status()).as_str()));
    }

    let bytes = resp.bytes()?;
    Ok(bytes)
}

