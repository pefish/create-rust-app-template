use super::test1::test1;

pub async fn test_method() -> String {
    return test1().await;
}
