// 实现一个和 mini-redis-server 进行交互的 client
// 1.识别用户输入,将输入转换成 frame,并写入 stream
// 2.读取响应
// 3.打印出值
// the first edition,
// use stdin(),and read to a buffer,convert the buffer to a vec,then match it's elements,according every branch to handle separate

// the second edition
// use the clap-builder,create a struct which contain command ,then call the parse(),match the command,handle the branch separate

// use std::io::{self, stdout, Write};
// mod set;
// mod get;
// use cli_for_miniredis::input_handle;
// use input_handle::handle;
// use set::do_set;
// use get::do_get;

// const ADDR:usize = "127.0.0.1:6379";
// #[tokio::main]
// async fn main() {
//     loop {
//         let mut input = String::new();
//         let user_hint = r"\e\d";
//         println!("Please insert {user_hint} between command and each element!");
//         print!("> ");
//         let _ = stdout().flush();
//         if let Err(err) = io::stdin().read_line(&mut input) {
//             eprint!("readline error: {err}");
//             return ;
//         };

//         let lowercase_input = input.trim().to_lowercase();
//         let input_vec:Vec<&str> = lowercase_input.split(r"\e\d").collect();

// use std::ffi::OsString;
//         handle(input_vec).await;
//     }
// }
//  use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
use clap::Parser;
use cli_for_miniredis::{do_get, do_set};
use std::io::{stdin, stdout, Write};

#[derive(Parser, Debug)]
#[command(no_binary_name(true))]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("reach to line 59");

    // let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    println!("reach to line 62");


    loop {
        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input)?;
        println!("input: {input}");

        // let command = Command::parse_from(input.clone().trim().split(' '));
        match Command::try_parse_from(input.clone().trim().split(' ')) {
            Ok(command) => {
                println!("parse the command: {command:?}");
                match command {
                    Command::Get { key } => do_get(&key).await?,

                    Command::Set { key, value } => do_set(&key, &value).await?,
                }
            }
            Err(_) => {
                eprintln!("Please input a valid command!\r\nExample: get key\r\n         set key value");
                continue;
            }
        }
    }
}
