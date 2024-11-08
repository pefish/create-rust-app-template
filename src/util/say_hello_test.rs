#[cfg(test)]
use super::say_hello::hello;

#[tokio::main]
#[test]
async fn hello_test() {
    assert_eq!(hello().await, "hello");
}
