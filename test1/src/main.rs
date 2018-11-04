//fn print_number(x: i32) {
//    println!("x is: {}", x);
//}

//fn print_sum(x: i32, y : i32) {
//    println!("sum is: {}", x + y);
//}

//fn add_one(x: i32) -> i32 {
//    x + 1
//}

// 错误的做法,因为带上分号,会返回一个() ????
//fn add_one(x: i32) -> i32 {
//    x + 1;
//}

// 基于表达式的语句,一切都是表达式, return 是一个非常糟糕的做法
//fn add_one2(x: i32) -> i32 {
//    return x + 1
//}

// 发散函数
//fn diverges() -> ! {
//    panic!("This function never returns!");
//}

//fn plus_one(i: i32) -> i32 {
//    i + 1
//}

fn main() {

    // 定义方法1: 推导类型
//    let x1 = 5;

    // 多个变量同时赋值
//    let (x2, x3) = (1, 2);

    // 定义方法2: 声明类型
//    let x4: i32 = 5;

    // 可变变量
//    let mut x5 = 5;

    // 定义方法3: 不初始化值
//    let x6: i32;

// 作用域
//    {
//        let x7 = 7;
//    }
//
//    println!("{}", x7)


// 作用域隐藏
//    let x8 = 8;
//
//    {
//        println!("{}", x8);
//        let x8 = 12;
//        println!("{}", x8);
//    }
//
//    println!("{}", x8);
//    let x8 = 42;
//    println!("{}", x8);

// 函数
//    let x9 = 9;
//
//    print_number(x9);

// 返回值
//    let x10 = 10;
//    print_number(add_one(x10));

// 发散函数可以用作一切类型的返回值
//    let _x11: i32 = diverges();

    // 函数指针
//    let f: fn(i32) -> i32 = plus_one;
//
//    let f2 = plus_one;
//
//    f(1);
//    f2(2);

    // bool 类型
//    let x = true;
//    let y: bool = false;

    // char 类型(注意是4个字节)
//    let x = 'x';
//    let two_hearts = '💕';

    // 基础类型
    // i8 i16 i32 i64 u8 u16 u32 u64 isize usize f32 f64

    // 数组
//    let a = [1, 2, 3];
//
//    let mut m = [1, 2, 3];

    // 数组值简写, 打印长度
//    let a = [0; 20];
//
//    println!("{}", a.len());


    // 切片
//    let a = [0, 1, 2, 3, 4];
//
//    let complete = &a[..];
//
//    let middle = &a[1..4];
//
//    println!("{}", middle.len());

    // 元组
//    let x = (1, "hello");
//
//    let y: (i32, &str) = (1, "hello");

    // 元组索引

//    let tuple = (1, 2, 3);
//
//    let x = tuple.0;
//    let y = tuple.1;
//    let z = tuple.2;

    // 函数类型
//    fn foo(x:i32) -> i32{ x}
//
//    let x: fn(i32) -> i32 = foo;


    // if else 语句
//    let x = 5;
//
//    if x == 5 {
//
//    } else if x == 6 {
//
//    } else {
//
//    }

    // if else 特殊
//    let x = 5;
//
//    let y = if x == 5 {
//        10
//    } else {
//        15;
//    };

    // 无限循环
//    loop {
//
//    }


}
