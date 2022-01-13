use super::request::request;
use serde_json::Value;
use serde::Deserialize;

static PREFIX: &str = "https://api.juejin.cn/growth_api/v1";

#[derive(Debug, Deserialize)]
pub struct ResData {
  pub data: Value,
  pub err_msg: String,
  pub err_no: i32,
}

/// 获取签到状态
pub async fn get_status() -> Result<ResData, String> {
  match request(
    format!("{}/get_today_status", PREFIX).as_str(),
    None,
    None
  ).await {
    Ok(res) => {
      let buf = hyper::body::to_bytes(res).await.unwrap();
      let v: Result<ResData, _> = serde_json::from_slice(&buf);
      match v {
        Ok(s) => {
          Ok(s)
        },
        Err(e) => {
          println!("getStatus: fail: {:?}", e);
          Err("解析格式失败 fail".to_string())
        }
      }
    },
    Err(e) => {
      println!("getStatus: fail: {:?}", e);
      Err("获取签到状态 fail".to_string())
    }
  }
}

/// 签到
pub async fn check_in() -> Result<ResData, String> {
  match request(
    format!("{}/check_in", PREFIX).as_str(),
    Some("POST"),
    None
  ).await {
    Ok(res) => {
      let buf = hyper::body::to_bytes(res).await.unwrap();
      let v: Result<ResData, _> = serde_json::from_slice(&buf);
      match v {
        Ok(s) => {
          Ok(s)
        },
        Err(e) => {
          println!("check_in: fail: {:?}", e);
          Err("check_in fail".to_string())
        }
      }
    },
    Err(e) => {
      println!("check_in: fail: {:?}", e);
      Err("签到 fail".to_string())
    }
  }
}

/// 获取连续签到天数、累计签到天数
pub async fn get_counts() -> Result<ResData, String> {
  match request(
    format!("{}/get_counts", PREFIX).as_str(),
  None,
    None
  ).await {
    Ok(res) => {
      let buf = hyper::body::to_bytes(res).await.unwrap();
      let v: Result<ResData, _> = serde_json::from_slice(&buf);
      match v {
        Ok(s) => {
          Ok(s)
        },
        Err(e) => {
          println!("get_counts: fail: {:?}", e);
          Err("get_counts fail".to_string())
        }
      }
    },
    Err(e) => {
      println!("get_counts: fail: {:?}", e);
      Err("获取连续签到天数、累计签到天数 fail".to_string())
    }
  }
}

/// 获取当前积分
pub async fn get_cur_point() -> Result<ResData, String> {
  match request(
    format!("{}/get_cur_point", PREFIX).as_str(),
  None,
    None
  ).await {
    Ok(res) => {
      let buf = hyper::body::to_bytes(res).await.unwrap();
      let v: Result<ResData, _> = serde_json::from_slice(&buf);
      match v {
        Ok(s) => {
          Ok(s)
        },
        Err(e) => {
          println!("get_cur_point: fail: {:?}", e);
          Err("get_cur_point fail".to_string())
        }
      }
    },
    Err(e) => {
      println!("get_counts: fail: {:?}", e);
      Err("获取当前积分 fail".to_string())
    }
  }
}

/// 抽奖
pub async fn lottery() -> Result<ResData, String> {
  match request(
    format!("{}/lottery/draw", PREFIX).as_str(),
    Some("POST"),
    None
  ).await {
    Ok(res) => {
      let buf = hyper::body::to_bytes(res).await.unwrap();
      let v: Result<ResData, _> = serde_json::from_slice(&buf);
      match v {
        Ok(s) => {
          Ok(s)
        },
        Err(e) => {
          println!("lottery: fail: {:?}", e);
          Err("lottery fail".to_string())
        }
      }
    },
    Err(e) => {
      println!("lottery: fail: {:?}", e);
      Err("抽奖 fail".to_string())
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[tokio::test]
  async fn it_should_work() {
    get_status().await;
    check_in().await;
    lottery().await;
    get_cur_point().await;
    get_counts().await;
  }
}