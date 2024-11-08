use crate::util::say_hello::hello;

pub async fn test1() -> String {
    return hello().await;
}
