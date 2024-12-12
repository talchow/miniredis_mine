// 实现一个和 mini-redis-server 进行交互的 client
// 1.识别用户输入,将输入转换成 frame,并写入 stream
// 2.读取响应
// 3.打印出值

use std::io::{self, stdout, Write};
mod set;
mod get;
mod input_handle;
use input_handle::input_handle;
use set::do_set;
use get::do_get;

// const ADDR:usize = "127.0.0.1:6379";
#[tokio::main]
async fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprint!("readline error: {err}");
            return ;
        };
        
        let lowercase_input = input.trim().to_lowercase();
        let input_vec:Vec<&str> = lowercase_input.split(' ').collect();

        input_handle(input_vec).await;
    }
}

