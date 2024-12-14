use crate::do_get;
use crate::do_set;
pub async fn handle(input_vec: Vec<&str>) {
    let _ = match input_vec.len() {
        2 => match input_vec.clone().into_iter().next() {
            Some("get") => do_get(input_vec[1]).await,
            _ => {
                println!("error input");
                return;
            }
        },
        3 => match input_vec.clone().into_iter().next() {
            Some("set") => do_set(input_vec[1], input_vec[2]).await,
            _ => {
                println!("error input");
                return;
            }
        },

        _ => {
            return;
        }
    };
}
