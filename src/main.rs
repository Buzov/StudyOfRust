use std::{thread, time};
use std::cmp::Ordering;

//fn main {}  ==  fn main() -> ()
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

    let p1 = Point::default();
    let p2 = Point::default();
    let p1_eq_p2 = p1 == p2;
    let p3 = Point{x: 1.0, y: 2.0};
    let p1_eq_p3 = p1 == p3;
    println!("p1 == p2: {}", p1_eq_p2);
    println!("p1 == p2: {}", p1_eq_p3);
}

fn number_type() -> () {
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

fn test_vec() {
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

fn test_ranges() {
    let bounded: std::ops::Range<i32> = 0..10;
    let from = 0..;
    let to = ..10;
    let full = ..;
    let inclusive = 0..=9;

    for i in (0..10).step_by(2) {
        println!("i = {}", i);
    }
}


fn test_loop() {
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

fn test_loop_2() {
    let x = 30;
    let init;

    if x == 31 {
        init = 92;
    } else {
        loop {}
    }
    println!("{}", init); // ok!
}

fn test_loop_3() {
    let x = 30;
    let init: i32 = loop {
        if  x == 31 {
            break 92;
        }
    };

    println!("{}", init); // ok!
}

fn test_expression() -> i32 {
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

#[derive(Default)]
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

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
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

//Паттерн newtype
//Представление в памяти такое же, как и у внутреннего типа
//Нет необходимости в аннотациях
//Нет автоматической конверсии/автоматических методов
//struct Kilometers(f64);
//struct Miles(f64);

// Struct-tuple
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

//Zero Sized Types
//unit struct
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

// Types tage
struct Kilometers;
struct Miles;

struct Distance<M> {
    amount: f64,
    metric: M,
}

fn test_types_struct() {
    let d1: Distance<Kilometers> = Distance {
        amount: 92.0,
        metric: Kilometers,
    };
    let d2: Distance<Miles> = Distance {
        amount: 92.0,
        metric: Miles,
    };
}

fn print_slice(xs: &[i32]) {
    for i in 0..xs.len() {
        println!("{}", xs[i]);
    }
}


//Dynamically Sized Types
//[i32; 4]  — четыре числа
//&[i32; 4]  — адресс в памяти, где лежат четыре числа
//[i32]  — n чисел
//&[i32]  — указатель + количество элементов, fat pointer
//mem::size_of::<[i32]>();
//^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
//assert_eq!(
//mem::size_of::<&[i32]>(),
//mem::size_of::<usize>() * 2,

enum Shape {
    Circle {
        center: Point,
        radius: f64,
    },
    Square {
        bottom_left: Point,
        top_right: Point,
    },
}

impl Shape {
    fn circle(center: Point, radius: f64) -> Shape {
        Shape::Circle { center, radius }
    }
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius, .. } => {
                std::f64::consts::PI * radius * radius
            }
            Shape::Square { bottom_left, top_right } => {
                unimplemented!()
            }
        }
    }
}

enum Expr {
    Negation(Box<Expr>),
    BinOp { lhs: Box<Expr>, rhs: Box<Expr> },
    Unit,
}

// use std::cmp::Ordering;
//enum Ordering {
//    Less,
//    Equal,
//    Greater,
//}

fn binary_search(xs: &[i32], x: i32) -> bool {
    if xs.is_empty() {
        return false;
    }
    let mid = xs.len() / 2;
    let subslice = match xs[mid].cmp(&x) {
        Ordering::Less => &xs[mid + 1..],
        Ordering::Equal => return true,
        Ordering::Greater => &xs[..mid],
    };
    binary_search(subslice, x)
}

fn foo2(xs: &[i32]) {
    match xs.get(92) {
        Some(value) => println!("Some"),
        None => panic!("out of bounds access")
    }
}

//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}
//impl<T> [T] {
//    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
//        where
//            T: Ord
//    {
//        self.binary_search_by(|p| p.cmp(x))
//    }
//}

//Newtype Variant
//BinOp  и If  типами не являются
enum Expr1 {
    BinOp {
        lhs: Box<Expr1>,
        rhs: Box<Expr1>,
//        op: Op,
    },
    If {
        cond: Box<Expr1>,
        then_branch: Box<Expr1>,
        else_branch: Box<Expr1>,
    },
}

enum Expr2 {
    BinOp(BinOp),
    If(If)
}

struct BinOp {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
//    op: Op,
}

struct If {
    cond: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
}

//энум без вариантов — аналог  !
//size_of::<Void>() == 0 математически должна быть -бесконечность
enum Void {}

fn test_enum_void(void: Void) -> i32 {
    let x =  match void {
    };
    x
}
//гарантированно что
//mem::size_of::<Option<&T>>() == mem::size_of::<&T>()
//mem::size_of::<Option<Box<T>>>() == mem::size_of::<Box<T>>()
//bool  занимает 1 байт, чтобы  &bool  работал всегда
//Newtype Variant может быть больше обычного enum из-за паддинга :o)
//бывают типы "страных" размеров: ZST, DST, uninhabited
