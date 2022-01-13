use hyper::{Request, Client, client, Body};
use hyper_tls::HttpsConnector;
use super::config::COOKIE;


pub fn request(
  uri: &str,
  method: Option<&str>,
  body: Option<Vec<u8>>
) -> client::ResponseFuture {

  let body = match body {
    Some(byt) => Body::from(byt),
    None => Body::empty(),
  };

  let method= method.unwrap_or("GET");

  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);

  client.request(
    Request::builder()
    .header("cookie", COOKIE.as_str())
    .header("accept", "*/*")
    .header("accept-language", "zh-CN,zh;q=0.9")
    .header("content-type", "application/json")
    .header("sec-ch-ua", "\" Not;A Brand\";v=\"99\", \"Google Chrome\";v=\"97\", \"Chromium\";v=\"97\"")
    .header("sec-ch-ua-mobile", "?0")
    .header("sec-ch-ua-platform", "\"Windows\"")
    .header("sec-fetch-dest", "empty")
    .header("sec-fetch-mode", "cors")
    .header("sec-fetch-site", "same-site")
    .header("Referer", "https://juejin.cn/")
    .header("Referrer-Policy", "strict-origin-when-cross-origin")
    .method(method)
    .uri(uri)
    .body(body)
    .unwrap()
  )
}