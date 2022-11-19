use crate::BoxResult;
use anyhow::Result as AnyResult;
use clap::{Arg, ArgMatches};
use std::{
    error::Error,
    fmt::format,
    path::{self, Path, PathBuf},
};
use tokio_test;

#[derive(Debug, Default)]
pub struct DvscArgs {
    name: String,
    pub is_download: bool,
    pub ori_url: String,
}

pub const download_dir: &'static str = "Downloads";

pub fn app() -> clap::Command<'static> {
    let ori_url_arg = Arg::new("url").value_name("url").index(2);
    let download_flag = Arg::new("download").required(false).default_value("false");

    let cmd = clap::Command::new("dvsc")
        .arg(download_flag)
        .arg(ori_url_arg);

    return cmd;
}

pub fn matches() -> ArgMatches {
    app().get_matches()
}

impl DvscArgs {
    pub fn parse(matches: ArgMatches) -> BoxResult<DvscArgs> {
        let is_download = matches
            .value_of("download")
            .map_or(false, |v| v.parse::<bool>().unwrap_or_default());
        let ori_url = matches.value_of("url").unwrap_or_default();

        Ok(DvscArgs {
            name: "dvsc".to_string(),
            ori_url: ori_url.to_string(),
            is_download,
        })
    }

    pub fn trans_url(&self) -> String {
        // ori url: https://az764295.vo.msecnd.net/stable/3b889b090b5ad5793f524b5d1d39fda662b96a2a/code_1.69.2-1658162013_amd64.deb
        trans_url(self.ori_url.to_string())
    }
}

pub fn trans_url(url: String) -> String {
    url.replace("az764295.vo.msecnd.net", get_new_dl_host())
}

pub fn get_new_dl_host() -> &'static str {
    return "vscode.cdn.azure.cn";
}

pub async fn get_res() -> AnyResult<String> {
    let default_url = "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64";
    let resp = reqwest::get(default_url).await?;
    let new_url = format!("https://{}{}", get_new_dl_host(), resp.url().path());
    Ok(new_url)
}

pub async fn download_vscode(url: &str) -> AnyResult<()> {
    let filename = std::path::Path::new(url)
        .file_name()
        .map_or("vscode.deb", |s1| s1.to_str().unwrap());
    match std::env::home_dir() {
        Some(home_dir) => {
            // dir: ~/Downloads
            let target = format!(
                "{}/{}/{}",
                home_dir.to_str().unwrap(),
                download_dir,
                filename
            );
            // dbg!(&target);
            std::fs::File::create(&target)?;
            let resp = reqwest::get(url).await?;
            std::fs::write(target, resp.bytes().await?)?;
            return Ok(());
        }
        _ => {
            return Err(anyhow::format_err!("Home dir is invalid."));
        }
    }
}

/// just for test
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dl_1() {
        let url = "https://vscode.cdn.azure.cn/stable/6261075646f055b99068d3688932416f2346dd3b/code_1.73.1-1667967334_amd64.deb";
        let a = aw!(download_vscode(url));
        eprintln!("{:?}", a);
        assert!(a.is_ok());
    }
}
