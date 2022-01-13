use juejing_actions::service::{
    get_counts,
    get_cur_point,
    get_status,
    check_in,
};

async fn get_info() {
    tokio::spawn(async move {
       match get_counts().await {
           Ok(res) => {
               let data = res.data.as_object().unwrap();
               println!("连续签到天数: {}", data.get("cont_count").unwrap());
               println!("累计签到天数: {}", data.get("sum_count").unwrap());
           },
           Err(e) => {
               println!("{}", e);
           }
       }
    }).await.unwrap();
    tokio::spawn(async move {
        match get_cur_point().await {
            Ok(res) => {
                println!("当前矿石数: {:?}", res.data);
            },
            Err(e) => {
                println!("{}", e);
            }
        }
     }).await.unwrap();
}

#[tokio::main]
async fn main() {
    match get_status().await {
        Ok(res) => {
            if res.err_no == 0 && !res.data.as_bool().unwrap() {
                // 签到
                match check_in().await {
                    Ok(check_in_res) => {
                        println!("签到：{}", check_in_res.err_msg);
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            } else {
                println!("今日已签到，请勿重复签到");
            }
            get_info().await;
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
