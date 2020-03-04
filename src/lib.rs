// lib.rs 是约定好的 crate 入口文件，使项目成为 crate

pub mod util; // 导出 util 模块

pub use util::calc; // pub use 可以导出 calc 模块
