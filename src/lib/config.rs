use once_cell::sync::Lazy;
use std::env;

pub static COOKIE: Lazy<String> = Lazy::new(|| {
  match env::var("JUEJIN_COOKIE") {
    Ok(str) => str,
    Err(_) => {
      panic!("JUEJIN_COOKIE is not found");
    }
  }
});