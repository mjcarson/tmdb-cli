#[macro_export]
macro_rules! client {
  () => {
    reqwest::Client::builder()
      .timeout(std::time::Duration::from_secs(30))
      .build()
      .expect("failed to build client")
  }
}

#[macro_export]
macro_rules! opt_param {
  ($params:expr, $name:expr, $opt:expr) => {
    if let Some(opt) = $opt.as_ref() {
      $params.push(($name, opt));
    }
  }
}

