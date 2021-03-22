use anyhow::Result;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use std::env;
use std::fs;
use std::path::Path;

const GEOIP_DATA_URL: &str =
    "https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/geoip.dat";
const GEOSITE_DATA_URL: &str =
    "https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/geosite.dat";

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("geodata_util", LevelFilter::Info)
        .init()?;

    let file_geoip = env::current_dir()?.join("geoip.dat");
    let file_geosite = env::current_dir()?.join("geosite.dat");

    // 更新geoip.dat
    update_geo_data(GEOIP_DATA_URL, file_geoip).expect("download geoip.dat failed");
    info!("update geoip.dat success");

    // 更新geosite.dat
    update_geo_data(GEOSITE_DATA_URL, file_geosite).expect("download geosite.dat failed");
    info!("update geoip.dat success");

    Ok(())
}

fn update_geo_data<P: AsRef<Path>>(url: &str, path: P) -> Result<()> {
    // TODO 备份旧数据

    let save = path.as_ref().display();

    // 下载文件
    info!("downloading from {}", url);

    let resp = reqwest::blocking::get(url)?;

    if !resp.status().is_success() {
        let err = format!("response code: {}", resp.status());
        panic!(err);
    }

    // 写入文件
    info!("save to {}", save);
    fs::write(path, &resp.bytes()?)?;

    // 完成
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_geoip_file() {
        let f = std::env::current_dir().unwrap().join("geoip.dat");

        let ret = update_geo_data(GEOIP_DATA_URL, f);

        assert!(ret.is_ok());
    }

    #[test]
    fn test_update_geosite_file() {
        let f = std::env::current_dir().unwrap().join("geosite.dat");

        let ret = update_geo_data(GEOSITE_DATA_URL, f);

        assert!(ret.is_ok());
    }
}
