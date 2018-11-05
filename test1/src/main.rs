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

//fn take(v: Vec<i32>) {
//
//}

//fn double(x: i32) -> i32 {
//    x * 2
//}

//fn change_truth(x: bool) -> bool {
//    !x
//}


// 需要返回所有权
//fn foo(v: Vec<i32>) -> Vec<i32> {
//    v
//}


// 更多的变量需要返回所有权时将会变得非常的糟糕
//fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
//    (v1, v2, 42)
//}

// 借用,更好的例子
//fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
//    42
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

    // while 循环
//    let mut x = 5;
//
//    let mut done = false;
//
//    while !done {
//        x += x - 3;
//
//        println!("{}", x);
//
//        if x % 5 == 0 {
//            done = true;
//        }
//    }

    // for 循环
//    for x in 0..10 {
//        println!("{}",x);
//    }

//    let array = [0, 1, 2, 3];
//
//    for var in array.iter() {
//        println!("{}", var);
//    }

    // 范围
//    for (index, value) in (5..10).enumerate() {
//        println!("index = {} and value = {}", index, value);
//    }

    // 行级别打印
//    let lines = "hello\nworld".lines();
//
//    for (linenumber, line) in lines.enumerate() {
//        println!("{}: {}", linenumber, line);
//    }

    // break continue c++ go 一样

    // 类似goto语句
//    'outer: for x in 0..10 {
//        'inner: for y in 0..10 {
//            if x % 2 == 0 { continue 'outer;}
//            if y % 2 == 0 { continue 'inner;}
//            println!("x: {} y: {}", x, y);
//        }
//    }

    // vector
//    let v = vec![1, 2, 3, 4, 5];

    // 10 个 zero 值
//    let v = vec![0; 10];

    // 越界访问,会触发panic
//    println!("Item 7 is {}", v[7]);

    // 处理越界错误
//    match v.get(7) {
//        Some(x) => println!("Item 7 is {}", x),
//        None => println!("Sorry, this vector is too short.")
//    };

    // 迭代vector 元素

//    let mut v = vec![1, 2, 3, 4, 5];
//
//    // 1. 引用迭代
//    for i in &v {
//        println!("A reference to {}", i);
//    }
//
//    // 2. 可变引用迭代
//    for i in &mut v {
//        println!("A mutable reference to {}", i);
//    }
//
//    // 3. 获取所有权 (所有权被获得之后无法再次获取所有权)
//    for i in v {
//        println!("Take ownership of the vector and its element {}", i);
//    }

    // rust对于任何给定的资源都正好只有一个绑定与之对应

//    let v = vec![1, 2, 3];
//
//    let v2 = v;
//
//    // 尝试使用v会引发错误, 因为所有权被转移了 (vector 栈上存有实际数据的指针, 栈上数据和堆上数据实时同步会有问题,违反Rust的安全保证)
//    println!("{}", v[0]);

    // 同样所有权被转移了
//    let v = vec![1, 2, 3];
//
//    take(v);
//
//    println!("{}", v[0]);

//    let  v = vec![1, 2, 3];
//
//    let mut v2 = v;
//
//    v2[1] = 100;
//
//    println!("{}", v[1]);

    // 普通类型是copy的
//    let v = 1;
//    let v2 = v;
//
//    println!("v is: {}", v);


    // 传到函数还是会copy的
//    let a = 5;
//
//    let _y = double(a);
//
//    println!("{}" ,a);

//    let a= true;
//
//    let _y = change_truth(a);
//
//    println!("{}", _y);

    // 交还所有权
//    let v1 = vec![1, 2, 3];
//    let v2 = vec![1, 2, 3];
//
//    let (v1, v2, answer) = foo(v1, v2);

    // 交换所有权更好的方法 (借用)
//    let v1 = vec![1, 2, 3];
//    let v2 = vec![1, 2, 3];
//
//    let answer = foo(&v1, &v2);


    // TODO |a, &b| a + b 是什么意思呀
//    fn sum_vec(v: &Vec<i32>) -> i32 {
//        return v.iter().fold(0, |a, &b| a + b);
//    }
//
//    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
//        let s1 = sum_vec(v1);
//        let s2 = sum_vec(v2);
//        s1 + s2
//    }
//
//    let v1 = vec![1, 2, 3];
//    let v2 = vec![4, 5, 6];
//
//    let answer = foo(&v1, &v2);
//
//    println!("{}", answer);

    // 引用是不能改变的
//    fn foo(v: &Vec<i32>) {
//        v.push(5);
//    }
//
//    let v = vec![];
//
//    foo(&v);

//    let mut x = 5;
//
//    {
//        let y = &mut x;
//        *y += 1;
//    }
//    println!("{}", x);

    // 同一个作用域下,要么只有一个对资源A的可变引用(&mut T), 要么有N个不可变引用(&T)

//    let mut x = 5;
//
//    let y = &mut x;
//
//    *y += 1;
//
//    println!("{}", x);

    // 不同作用域就可以编译了
//    let mut x = 5;
//
//    {
//        let y = &mut x;
//        *y += 1;
//    }
//
//    println!("{}", x);

    // 引用必须与它引用的值存活的一样长
//    let y: &i32;
//
//    {
//        let x = 5;
//        y = &x;
//    }

//    let y: &i32;
//
//    let x = 5;
//
//    y = &x;

//    let r;
//
//    {
//        let i = 1;
//        r = &i;
//    }

    // 生命周期推导
//    fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
//        line
//    }
//
//
//    let line = "lang:en=Hello World!";
//    let lang = "en";
//
//    let v;
//
//    {
//        let p = format!("lang:{}=", lang);
//        v = skip_prefix(line, p.as_str());
//    }
//
//    println!("{}", v);

    // &mut i32  :  一个i32的可变引用
    // &'a mut i32: 一个带有生命周期'a的i32的可变引用

    // struct 的生命周期
//    struct Foo<'a> {
//        x : &'a i32,
//    }
//
//    let y = &5;  // 等于 let _y = 5; let y = &_y;
//
//    let f = Foo { x: y };
//    println!("{}", f.x);

    // 实现一个方法
//    struct Foo<'a> {
//        x: &'a i32,
//    }
//
//    impl<'a> Foo<'a> {
//        fn x(&self) -> &'a i32 { self.x }
//    }
//
//    let y = &5;
//
//    let f = Foo { x: y };
//
//    println!("x is :{}", f.x());


    // 多次使用同一个生命周期
//    fn x_or_y<'a>(x :&'a str, y: &'a str) -> &'a str {
//        x
//    }

    // 使用不同的生命周期
//    fn x_or_y<'a,'b> (x  : &'a str, y : &'b str) -> &'a str {
//        x
//    }

    // static 横跨整个程序的生命周期
    let x: &'static str = "Hello world.";

    // 全局变量
    static FOO: i32 = 5;
    let x: &'static i32 = &FOO;


}