use anyhow::Result as AnyResult;

mod cli;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() {
    match cli::app::DvscArgs::parse(cli::app::matches()) {
        Ok(args) => {
            let res = cli::handle::handle_url_arg(args).await;
            assert!(res.is_ok());
        }
        Err(err) => {
            eprintln!("参数错误：{}，请输入正确的原始下载链接！", err)
        }
    }
}
