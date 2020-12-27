#[macro_export]
macro_rules! get {
  ($handler:expr, $req:expr) => {
    $handler.client.execute($req.build()?).await?.error_for_status()
  }
}

pub struct Core {
  /// The URL/ip to reach tmdb at
  pub host: String,
  /// A token to use when authenticating
  pub token: String,
  /// A client
  pub client: reqwest::Client,
}

impl Core {
  pub fn new(host: String, token: String) -> Self {
    // build a reqwest client
    let client = reqwest::Client::builder()
      .timeout(std::time::Duration::from_secs(30))
      .build()
      .expect("failed to build client");
      Core { host, token, client }
  }
}
