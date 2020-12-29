#[macro_export]
macro_rules! get {
    ($handler:expr, $req:expr) => {
        $handler
            .client
            .execute($req.build()?)
            .await?
            .error_for_status()
    };
}
