use std::{thread, time};

pub(super)fn number_type() -> () {
    let y = 92_000_000i64;
    println!("y: {}", y);

    let hex_octal_bin : i64 = 0xffff_ffff + 0o777 + 0b1;
    println!("{}", hex_octal_bin);
    let byte: u8 = b'a';
    println!("byte: {}", byte);
//    assert_eq!(byte, 65);
    println!("{} - {}", 8.5f32.ceil().sin().round().sqrt(), 60f64.sin());

//    let f64_nan = f64::NAN;
//    println!("f64::NAN: {}", f64_nan);
}

fn array() {
    let xs: [u8; 3] = [1, 2, 3];
    assert_eq!(xs[0], 1); // index -- usize
    assert_eq!(xs.len(), 3); // len() -- usize
    let mut buf = [0u8; 1024]; // буфер из 1024 элементов, заполненный нулями
}

pub(super)fn links() {
    let mut x: i32 = 92;
    let r: &mut i32 = &mut 92; // явное взятие ссылки
    *r += 1; // явное разыменовывание ссылки
    println!("r: {}", r);
    let a = 5;
    let b = &a;
    println!("{}", b);
    println!("{}", a);
}

pub(super)fn box_test() {
    let x: Box<i32> = Box::new(92);
}

fn foo() {
    let a: Box<i32>;
    a = Box::new(1);
    bar(&*a)
}

fn bar(b: &i32) {
}

pub(super)fn test_vec() {
    for x in vec![1, 2, 3] {
        println!("x = {}", x);
    }

    let xs  = vec![1, 2, 3];
    for i in 0..xs.len() {
        let x = xs[i];
        println!("x = {}", x);
    }
    let xs2  = vec![1, 2, 3];
    print_slice(&xs2);
}

pub(super)fn loops() {
    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    for (i,j) in (5..10).enumerate() {
        println!("i = {} и j = {}", i, j);
    }

    let lines = "привет\nмир\nhello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    let x: () = while false {};
    let x2: () = if true {  92; };
//    let x3 = if true {  92 }; error ожидается тип () если нет блока else

//    while true  {
//        if cond1 {
//            continue;
//        }
//        if cond2 {
//            break;
//        }
//    }
//
//    'outer: while cond1 {
//        while cond2 {
//            break 'outer;
//        }
//    }

}

pub(super)fn test_ranges() {
    let bounded: std::ops::Range<i32> = 0..10;
    let from = 0..;
    let to = ..10;
    let full = ..;
    let inclusive = 0..=9;

    for i in (0..10).step_by(2) {
        println!("i = {}", i);
    }
}


pub(super)fn test_loop() {
//    let uninit;
//    while true {
//        if condition {
//            uninit = 92;
//            break;
//        }
//    }
//    pritnln!("{}", uninit); // error
    let x = 30;
    let init;
    loop {
        if x == 31 {
            init = 92;
            break;
        }
    }
    println!("{}", init); // ok

}

pub(super)fn test_loop_2() {
    let x = 30;
    let init;

    if x == 31 {
        init = 92;
    } else {
        loop {}
    }
    println!("{}", init); // ok!
}

pub(super)fn test_loop_3() {
    let x = 30;
    let init: i32 = loop {
        if  x == 31 {
            break 92;
        }
    };

    println!("{}", init); // ok!
}

pub(super)fn test_expression() -> i32 {
    let x = 30;
    if x == 0 {
        println!("zero");
    }      // statement

    { 0; } // statement



    let s = if x > 0 {
        "positive"
    } else {
        "negative"
    };

    if true { 92 } else { 62 } // expression!
}

pub(super)fn test_closures() {
    let square = |x| x * x;
    assert_eq!(square(5), 25);
}

pub(super)fn test_fn_like_parameter() {
    fn plus_one(i: i32) -> i32 { i + 1 }

    let f = plus_one;
    let six = f(5);
    println!("{}", six);
    println!("fn_like_parameter result: {}", fn_like_parameter(f, 10));
    println!("fn_like_parameter result: {}", fn_like_parameter(f, 20));
}

fn fn_like_parameter(func: fn(i32) -> i32, i: i32) -> i32 {
    func(i)
}

pub(super)fn test_life_time_in_fn() {
    let x = 1;
    let r: &i32;
    {
        let y = 2;
        r = life_time_in_fn(&x, &y); // ok
    }
    println!("{}", *r);
    //    let x = 1;
//    let r: &i32;
//    let z: i32;
//    {
//        let y = 2;
//        r = &y; // borrowed value does not live long enough
//        z = y;
//    }
//    println!("{}", z);
//    println!("{}", *r);
}

fn life_time_in_fn<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    println!("life_time_in_fn y: {}", y);
    // y если вернуть y - будет ошибка компиляции
    x // parameter and the return type are declared
    // with different lifetimes
}

fn block() {
    let i: i32 = { 1  };
    let i: ()  = { 1; };
}

fn exclamation_mark_0() -> i32 {
    let x: i32 = exclamation_mark_1();
    x
}

fn exclamation_mark_1() -> ! {
//    let i = return;
    let x = exclamation_mark_2();
    x
}

fn exclamation_mark_2() -> ! {
    loop {

    }
}

pub(super)fn test_fn_tuple() {
    let (x, y) = fn_tuple();
    println!("x: {}, y: {}", x, y);
    tuple();
}

fn fn_tuple<'a>() -> (i32, &'a str) {
    (1, "dfgd")
}

pub(super)fn tuple() {
    let pair: (f32, i32) = (0.0, 92);
    let one: (f32,) = (0.0,); // кортеж из одного элемента, нужна запятая
//    let (x, y) = pair;
    let (_x, _y) = pair;
    let x = pair.0;
    let y = pair.1;
    println!("{}", x);
    println!("{}", y);

    let t = (92,); // кортеж ничего не стоит. адресс обертки и элемента совпадают
    println!("{:?}", &t as *const (i32,)); // 0x7ffc6b2f6aa4
    println!("{:?}", &t.0 as *const i32); // 0x7ffc6b2f6aa4
}

fn tuple_fn(fun: &Fn() -> (f64,f64)) -> (f64,f64) {
    fun()
}

pub(super)fn test_sleep() {
    sleep(1000);
}

fn sleep(millis: u64) {
    let ten_millis = time::Duration::from_millis(millis);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
}

fn print_slice(xs: &[i32]) {
    for i in 0..xs.len() {
        println!("{}", xs[i]);
    }
}