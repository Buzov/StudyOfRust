use std::{thread, time};

fn main() {

    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
//    number_type();
    let hex_octal_bin : i64 = 0xffff_ffff + 0o777 + 0b1;

    println!("{}", hex_octal_bin);
    fn plus_one(i: i32) -> i32 { i + 1 }

    let f = plus_one;
    let six = f(5);
    println!("{}", six);
    println!("{}", test(f, 10));
    let (x, y) = corteg();
    println!("x: {}, y: {}", x, y);

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

    let a = 5;
    let b = &a;
    println!("{}", b);
    println!("{}", a);

    println!("{} - {}", 8.5f32.ceil().sin().round().sqrt(), 60f64.sin());
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
    cortage();
    links();
    main_2();

    let point = Point{x: 1.0, y: 2.0};
    println!("point  distance_from_origin: {}", point.distance_from_origin());
    let point_origin = Point::origin();
    println!("point_origin  distance_from_origin: {}", point_origin.distance_from_origin());
//    let point = Point{x: 1.0, y: 2.0, };
}

//fn number_type() -> () {
//    let y = 92_000_000i64;
//    let hex_octal_bin : i64 = 0xffff_ffff + 0o777 + 0b1;
//    let byte: u8 = b'a';
//    assert_eq!(byte, 65);
//}

fn cortage() {
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

fn array() {
    let xs: [u8; 3] = [1, 2, 3];
    assert_eq!(xs[0], 1); // index -- usize
    assert_eq!(xs.len(), 3); // len() -- usize
    let mut buf = [0u8; 1024]; // буфер из 1024 элементов, заполненный нулями
}

fn links() {
    let mut x: i32 = 92;
    let r: &mut i32 = &mut 92; // явное взятие ссылки
    *r += 1; // явное разыменовывание ссылки
    println!("r: {}", r);
}

fn box_test() {
    let x: Box<i32> = Box::new(92);
}

fn foo() {
    let a: Box<i32>;
    a = Box::new(1);
    bar(&*a)
}

fn bar(b: &i32) {
}

fn test(func: fn(i32) -> i32, i: i32) -> i32 {
    func(i)
}

fn corteg<'a>() -> (i32, &'a str) {
    (1, "dfgd")
}

fn main_2() {
    let x = 1;
    let r: &i32;
    {
        let y = 2;
        r = f(&x, &y); // ok
    }
    println!("{}", *r);
}

fn f<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
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
//    let x = return;
    let x = exclamation_mark_2();
    x
}

fn exclamation_mark_2() -> ! {
    loop {

    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point{x: 0.0, y: 0.0}
    }

    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}