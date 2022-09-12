use crate::BoxResult;
use anyhow::Result as AnyResult;
use clap::ArgMatches;

#[derive(Debug, Default)]
pub struct DvscArgs {
    name: String,
    ori_url: String,
}

// pub const download_dir: &'static str = "~/Downloads";

pub fn app() -> clap::Command<'static> {
    let ori_url_arg = clap::Arg::new("url").value_name("url").index(1);
    let cmd = clap::Command::new("dvsc").arg(ori_url_arg);

    return cmd;
}

pub fn matches() -> ArgMatches {
    app().get_matches()
}

impl DvscArgs {
    pub fn parse(matches: ArgMatches) -> BoxResult<DvscArgs> {
        let ori_url = matches.value_of("url").unwrap_or_default();

        Ok(DvscArgs {
            name: "dvsc".to_string(),
            ori_url: ori_url.to_string(),
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
