use once_cell::sync::Lazy;
use std::env;

pub static COOKIE: Lazy<String> = Lazy::new(|| {
  match env::var("JUEJIN_COOKIE") {
    Ok(str) => str,
    Err(_) => {
      panic!("JUEJIN_COOKIE is not found");
    }
  }
  // "_ga=GA1.2.291535959.1636447975; MONITOR_WEB_ID=d2180597-7257-4116-adbb-7b17971a27bd; _gid=GA1.2.1683284446.1642065515; passport_csrf_token_default=d59677133b9a746ef70a0441566d8924; passport_csrf_token=d59677133b9a746ef70a0441566d8924; sid_guard=b3730fbbfa6bca34e9ca480206673e3f%7C1642065544%7C5184000%7CMon%2C+14-Mar-2022+09%3A19%3A04+GMT; uid_tt=5e64b406abfd3dd16cf1b3e7dc7b912c; uid_tt_ss=5e64b406abfd3dd16cf1b3e7dc7b912c; sid_tt=b3730fbbfa6bca34e9ca480206673e3f; sessionid=b3730fbbfa6bca34e9ca480206673e3f; sessionid_ss=b3730fbbfa6bca34e9ca480206673e3f; sid_ucp_v1=1.0.0-KDIyNmY4YjBkZmVkNDI4MGNmYWU4Yjg5MjYxNzIxZjY1MmQ1ZDgwZTgKFgjHkIG__fXFARCI3f-OBhiwFDgIQAsaAmxmIiBiMzczMGZiYmZhNmJjYTM0ZTljYTQ4MDIwNjY3M2UzZg; ssid_ucp_v1=1.0.0-KDIyNmY4YjBkZmVkNDI4MGNmYWU4Yjg5MjYxNzIxZjY1MmQ1ZDgwZTgKFgjHkIG__fXFARCI3f-OBhiwFDgIQAsaAmxmIiBiMzczMGZiYmZhNmJjYTM0ZTljYTQ4MDIwNjY3M2UzZg; n_mh=TkwD5HO_4vK6BAJ8J6SlH0I-4hW9l34I48KGwLTf3Gs".to_string()
});