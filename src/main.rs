// main.rs 是约定好的执行入口

mod util; // 引入模块，类似 import

use util::calc; // 类似 C# 中的 using namespace

fn main() {
    println!("Hello, world!");
    println!("3+4={}", calc::add(3, 4));
}
