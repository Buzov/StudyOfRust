use std::{thread, time};

fn main() {

    test_sleep();
    number_type();
    test_fn_tuple();
    tuple();
    loops();
    links();

    test_life_time_in_fn();
    test_closures();

    test_point();
    test_point_tuple();
//    println!("tuple_fnin: {}", tuple_fn(tuple_struct));
    zero_sized_types();

}

fn number_type() -> () {
    let y = 92_000_000i64;
    let hex_octal_bin : i64 = 0xffff_ffff + 0o777 + 0b1;
    println!("{}", hex_octal_bin);
    let byte: u8 = b'a';
//    assert_eq!(byte, 65);
    println!("{} - {}", 8.5f32.ceil().sin().round().sqrt(), 60f64.sin());
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
    let a = 5;
    let b = &a;
    println!("{}", b);
    println!("{}", a);
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

fn loops() {
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
}

fn test_closures() {
    let square = |x| x * x;
    assert_eq!(square(5), 25);
}

fn test_fn_like_parameter() {
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

fn test_life_time_in_fn() {
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

fn test_point() {
    let point = Point{x: 1.0, y: 2.0};
    println!("point  distance_from_origin: {}", point.distance_from_origin());
    let point_origin = Point::origin();
    println!("point_origin  distance_from_origin: {}", point_origin.distance_from_origin());
//    let point = Point{x: 1.0, y: 2.0, };
}

fn test_fn_tuple() {
    let (x, y) = fn_tuple();
    println!("x: {}, y: {}", x, y);
    tuple();
}

fn fn_tuple<'a>() -> (i32, &'a str) {
    (1, "dfgd")
}

fn tuple() {
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

struct PointTuple(f64,f64,);

impl PointTuple {
    fn origin() -> Point {
        Point{x: 0.0, y: 0.0}
    }

    fn dist(self, other: PointTuple) -> f64 {
        let PointTuple(x1, y1) = self;
        let PointTuple(x2, y2) = other;
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}

fn test_point_tuple() {
    let tuple_struct = PointTuple(0.0, 1.0);
    assert_eq!(tuple_struct.0, 0.0);
}

fn tuple_fn(fun: &Fn() -> (f64,f64)) -> (f64,f64) {
    fun()
}

struct Zero;

fn zero_sized_types() -> () {
    let t = Zero;
    assert!(std::mem::size_of::<Zero>() == 0);
    assert!(std::mem::size_of::<(Zero, Zero)>() == 0);
    assert!(std::mem::size_of::<[Zero; 1024]>() == 0);
    assert!(std::mem::size_of::<()>() == 0);
}

fn test_sleep() {
    sleep(1000);
}

fn sleep(millis: u64) {
    let ten_millis = time::Duration::from_millis(millis);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
}



