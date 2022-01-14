use once_cell::sync::{Lazy, OnceCell};
use std::env;
use serde::Deserialize;

pub static COOKIE: Lazy<String> = Lazy::new(|| {
  match env::var("JUEJIN_COOKIE") {
    Ok(str) => str,
    Err(_) => {
      panic!("JUEJIN_COOKIE is not found");
    }
  }
});

#[derive(Debug)]
pub struct QyWeixinConfig {
  /** 企业id */
  pub corpid: String,
  /** 应用 secret */
  pub corpsecret: String,
  /** 应用id */
  pub agentid: String,
  /** 指定接收消息的成员，成员ID列表（多个接收者用‘|’分隔，最多支持1000个） */
  pub touser: String,
}

pub static QYWEIXINCONFIG: Lazy<QyWeixinConfig> = Lazy::new(|| {
  QyWeixinConfig {
    corpid: env::var("QY_WEIXIN_CORPID").unwrap_or_else(|_| String::from("")),
    corpsecret: env::var("QY_WEIXIN_CORPSECRET").unwrap_or_else(|_| String::from("")),
    agentid: env::var("QY_WEIXIN_AGEBTID").unwrap_or_else(|_| String::from("")),
    touser: env::var("QY_WEIXIN_TOUSER").unwrap_or_else(|_| String::from("@all")),
  }
});

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct QyAccessTokenMap {
  pub errcode: i32,
  pub errmsg: String,
  pub access_token: String,
  pub expires_in: i32,
}

pub static QY_ACCESS_TOKEN_MAP: OnceCell<QyAccessTokenMap> = OnceCell::new();