mod cli;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() {
    match cli::app::DvscArgs::parse(cli::app::matches()) {
        Ok(res) => {
            let res_url = res.trans_url();
            println!("{res_url}");
        }
        Err(err) => {
            eprintln!("参数错误：{}，请输入正确的原始下载链接！", err)
        }
    }
}
