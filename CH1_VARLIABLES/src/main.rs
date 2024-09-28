fn main() {
    // 不可变变量
    let _nice_count = 100;
    let _nice_number: i64 = 10;
    // nice_count = 1;
    // 声明可变变量
    let mut _count = 1;
    println!("_count : {}", _count);
    _count = 2;
    println!("_count : {_count}");
    // Shadowing
    let x = 5;
    {
        // 命名空间
        let x = 10;
        println!("inner x : {}", x);
    }   // 命名空间结束，x 被销毁
    println!("output x : {x}");
    let x = "hello";
    println!("New x : {x}");
    let mut x = "this";
    println!("x : {x}");
    x = "that";
    println!("x : {x}");
}
