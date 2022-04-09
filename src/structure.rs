use std::cmp::Ordering;

use crate::shapes::point::Point;

//Паттерн newtype
//Представление в памяти такое же, как и у внутреннего типа
//Нет необходимости в аннотациях
//Нет автоматической конверсии/автоматических методов
//struct Kilometers(f64);
//struct Miles(f64);

// Struct-tuple
struct StructTuple(f64, f64);

impl StructTuple {
    fn dist(self, other: StructTuple) -> f64 {
        let StructTuple(x1, y1) = self;
        let StructTuple(x2, y2) = other;
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}

pub(super) fn test_struct_tuple() {
    let tuple_struct = StructTuple(0.0, 1.0);
    assert_eq!(tuple_struct.0, 0.0);
}

// Types tage
struct Kilometers;
struct Miles;

struct Distance<M> {
    amount: f64,
    metric: M,
}

//Zero Sized Types
//unit struct
struct Zero;

pub(super) fn zero_sized_types() -> () {
    let t = Zero;
    assert!(std::mem::size_of::<Zero>() == 0);
    assert!(std::mem::size_of::<(Zero, Zero)>() == 0);
    assert!(std::mem::size_of::<[Zero; 1024]>() == 0);
    assert!(std::mem::size_of::<()>() == 0);
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
            Shape::Circle { radius, .. } => std::f64::consts::PI * radius * radius,
            Shape::Square {
                bottom_left: _,
                top_right,
            } => {
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
        None => panic!("out of bounds access"),
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
        op: Op,
    },
    If {
        cond: Box<Expr1>,
        then_branch: Box<Expr1>,
        else_branch: Box<Expr1>,
    },
}

enum Expr2 {
    BinOp(BinOp),
    If(If),
}

struct BinOp {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: Op,
}

struct If {
    cond: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
}

struct Op {
    x: i32,
}

//энум без вариантов — аналог  !
//size_of::<Void>() == 0 математически должна быть -бесконечность
enum Void {}

fn test_enum_void(void: Void) -> i32 {
    let x = match void {};
    x
}
//гарантированно что
//mem::size_of::<Option<&T>>() == mem::size_of::<&T>()
//mem::size_of::<Option<Box<T>>>() == mem::size_of::<Box<T>>()
//bool  занимает 1 байт, чтобы  &bool  работал всегда
//Newtype Variant может быть больше обычного enum из-за паддинга :o)
//бывают типы "страных" размеров: ZST, DST, uninhabited
