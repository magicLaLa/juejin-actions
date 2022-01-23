use juejing_actions::service::{
    get_counts,
    get_cur_point,
    get_status,
    check_in,
};
use juejing_actions::qyweixin_notify::{get_access_token, send_notify};
use tokio::try_join;

async fn get_info() -> String {
    let handle1 = tokio::spawn(async move {
       match get_counts().await {
           Ok(res) => {
               let data = res.data.as_object().unwrap();
               let str1 = format!("连续签到天数: {}", data.get("cont_count").unwrap());
               let str2 = format!("累计签到天数: {}", data.get("sum_count").unwrap());
               println!("{}", &str1);
               println!("{}", &str2);
               format!("{}\r\n{}", str1, str2)
           },
           Err(e) => {
               println!("{}", e);
               e
           }
       }
    });
    let handle2 = tokio::spawn(async move {
        match get_cur_point().await {
            Ok(res) => {
                let str1 = format!("当前矿石数: {}", res.data.as_i64().unwrap());
                println!("{}", &str1);
                str1
            },
            Err(e) => {
                println!("{}", e);
                e
            }
        }
     });
    match try_join!(handle1, handle2) {
        Ok((msg1, msg2)) => format!("{}\r\n{}", msg1, msg2),
        Err(e) => format!("error is: {:?}", e),
    }

}

#[tokio::main]
async fn main() {
    let mut out_message = String::from("");
    if get_access_token().await.is_ok() {
        match get_status().await {
            Ok(res) => {
                if res.err_no == 0 && !res.data.as_bool().unwrap() {
                    // 签到
                    match check_in().await {
                        Ok(check_in_res) => {
                            let str = format!("签到：{}", check_in_res.err_msg);
                            println!("{}", &str);
                            out_message = format!("{}\r\n{}", out_message, str);
                        },
                        Err(e) => {
                            println!("{}", &e);
                            out_message = format!("{}\r\n{}", out_message, e);
                        }
                    }
                } else {
                    println!("今日已签到，请勿重复签到");
                    out_message = format!("{}\r\n{}", out_message, "今日已签到，请勿重复签到");
                }
                let info_str = get_info().await;
                out_message = format!("{}\r\n{}", out_message, info_str);
                send_notify(&out_message).await;
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    };
}
