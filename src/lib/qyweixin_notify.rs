use super::config::{QyAccessTokenMap, QYWEIXINCONFIG, QY_ACCESS_TOKEN_MAP};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

pub async fn get_access_token() -> Result<(), String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = format!("https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}", QYWEIXINCONFIG.corpid, QYWEIXINCONFIG.corpsecret);
    match client
        .request(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
    {
        Ok(res) => {
          let buf = hyper::body::to_bytes(res).await.unwrap();
          let v: Result<QyAccessTokenMap, _> = serde_json::from_slice(&buf);
          if let Ok(data) = v {
            assert_eq!(QY_ACCESS_TOKEN_MAP.set(data), Ok(()));
          }
          Ok(())
        }
        Err(e) => {
            println!("get_access_token fail: {:#?}", e);
            Err("get_access_token: 请求失败".to_string())
        }
    }
}

#[derive(Debug, Deserialize)]
struct Notify {
  errcode: i32,
  errmsg: String,
  msgid: Option<String>,
}

pub async fn send_notify(notify: &str) {
  let access_token = &QY_ACCESS_TOKEN_MAP.get().unwrap().access_token;
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);
  let uri = format!("https://qyapi.weixin.qq.com/cgi-bin/message/send?access_token={}", access_token);

  let mut data: Vec<u8> = Vec::new();

  data.extend("{".as_bytes());
  data.extend("\"msgtype\":\"text\",".as_bytes());
  data.extend(format!("\"touser\":\"{}\",", QYWEIXINCONFIG.touser).as_bytes());
  data.extend(format!("\"agentid\":\"{}\",", QYWEIXINCONFIG.agentid).as_bytes());
  data.extend("\"text\":{".as_bytes());
  data.extend(format!("\"content\":\"{}\"", notify).as_bytes());
  data.extend("}".as_bytes());
  data.extend("}".as_bytes());

  match client
      .request(Request::builder()
      .method("POST")
      .header("Content-Type", "application/json; charset=UTF-8")
      .uri(uri)
      .body(Body::from(data))
      .unwrap())
      .await
  {
      Ok(res) => {
        let buf = hyper::body::to_bytes(res).await.unwrap();
        let v: Result<Notify, _> = serde_json::from_slice(&buf);
        match v{
          Ok(data) => {
            if data.errcode == 0 {
              println!("发送消息成功: {}", data.errmsg);
            } else {
              println!("发送消息失败: {:#?}", data);
            }
          },
          Err(e) => {
            println!("序列化 send_notify data fail: {:#?}", e);
          },
        }
      }
      Err(e) => {
          println!("send_notify fail: {:#?}", e);
      }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  // #[tokio::test]
  // async fn get_access_token_is_work() {
  //   get_access_token().await;
  // }

  #[tokio::test]
  async fn send_notify_is_work() {
    if get_access_token().await.is_ok() {
      send_notify("测试消息").await;
    }
  }
}