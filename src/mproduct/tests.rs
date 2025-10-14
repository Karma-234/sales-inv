#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handler_returns_product() {
        let resp = get_product_handler().await;
        assert_eq!(resp.code, 200);
        assert!(resp.data.is_some());
        let p = resp.data.unwrap();
        assert_eq!(p.name, "Sample Widget");
    }
}
// --- IGNORE ---
