#[cfg(test)]
use super::util::hello;

#[tokio::main]
#[test]
async fn hello_test() {
    assert_eq!(hello().await, "hello");
}
