// 24301006 黄飞鹏 2026-11-25
// 自己的应用程序：计算并打印斐波那契数列前 10 项
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("My own app by 24301006 HuangFeipeng: Fibonacci");
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for i in 0..10 {
        println!("fib({}) = {}", i, a);
        let t = a + b;
        a = b;
        b = t;
    }
    println!("Test fibonacci OK!");
    0
}
