use tokio;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::test]
async fn test_get_title() {
    let client = ShindanClient::new(ShindanDomain::En).unwrap();
    let result = client.get_title("1218842").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_submit_shindan() {
    let client = ShindanClient::new(ShindanDomain::En).unwrap();
    let result = client.submit_shindan("1218842", "test_user").await;
    assert!(result.is_ok());
}