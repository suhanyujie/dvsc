use super::app;
use anyhow::{Ok, Result as AnyResult};

pub async fn handle_url_arg(args: app::DvscArgs) -> AnyResult<()> {
    let mut res_url = args.trans_url();
    if res_url == "" {
        // 如果为空，则直接使用默认的下载地址 todo
        res_url = app::get_res().await.expect("err");
    }
    if args.is_download {
        let target_dir = app::download_vscode(&res_url).await?;
        println!("Ok，下载完成！请到 [{target_dir}] 查看。");
    } else {
        println!("Ok，请浏览器打开下载：{res_url}");
    }

    Ok(())
}
