use anyhow::Result as AnyResult;

mod cli;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() {
    match cli::app::DvscArgs::parse(cli::app::matches()) {
        Ok(res) => {
            let res_url = res.trans_url();
            if res_url == "" {
                // 如果为空，则直接使用默认的下载地址 todo
                get_res().await.expect("err");
            }
            println!("{res_url}");
        }
        Err(err) => {
            eprintln!("参数错误：{}，请输入正确的原始下载链接！", err)
        }
    }
}

async fn get_res() -> AnyResult<()> {
    let default_url = "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64";
    let resp = reqwest::get(default_url).await?;
    let new_url = format!(
        "https://{}{}",
        cli::app::get_new_dl_host(),
        resp.url().path()
    );
    println!("{:#?}", new_url);
    Ok(())
}
