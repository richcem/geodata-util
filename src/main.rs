use anyhow::Result;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use std::env;
use std::fs;
use std::path::Path;

const DOWNLOAD_LINK_GEOIP: &str =
    "https://github.com/v2fly/geoip/releases/latest/download/geoip.dat";
const DOWNLOAD_LINK_GEOSITE: &str =
    "https://github.com/v2fly/domain-list-community/releases/latest/download/dlc.dat";

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("geodata_util", LevelFilter::Info)
        .init()?;

    let file_geoip = env::current_dir()?.join("geoip.dat");
    let file_geosite = env::current_dir()?.join("geosite.dat");

    // 更新geoip.dat
    match update_geo_data(DOWNLOAD_LINK_GEOIP, file_geoip) {
        Ok(()) => println!("update geoip.dat success"),
        Err(e) => panic!(e),
    }

    // 更新geosite.dat
    match update_geo_data(DOWNLOAD_LINK_GEOSITE, file_geosite) {
        Ok(()) => println!("update geoip.dat success"),
        Err(e) => panic!(e),
    }

    Ok(())
}

fn update_geo_data<P: AsRef<Path>>(url: &str, path: P) -> Result<()> {
    // TODO 备份旧数据

    let save = path.as_ref().display();

    // 下载文件
    info!("downloading from {}", url);

    let bytes = reqwest::blocking::get(url)?.bytes()?;

    // 写入文件
    info!("save to {}", save);
    fs::write(path, bytes)?;

    // 完成
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_geoip_file() {
        let f = std::env::current_dir().unwrap().join("geoip.dat");

        let ret = update_geo_data(DOWNLOAD_LINK_GEOIP, f);

        assert!(ret.is_ok());
    }

    #[test]
    fn test_update_geosite_file() {
        let f = std::env::current_dir().unwrap().join("geosite.dat");

        let ret = update_geo_data(DOWNLOAD_LINK_GEOSITE, f);

        assert!(ret.is_ok());
    }
}
