fn main() {
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

//fn number_type() -> () {
//    let y = 92_000_000i64;
//    let hex_octal_bin : i64 = 0xffff_ffff + 0o777 + 0b1;
//    let byte: u8 = b'a';
//    assert_eq!(byte, 65);
//}


fn test(func: fn(i32) -> i32, i: i32) -> i32 {
    func(i)
}

fn corteg<'a>() -> (i32, &'a str) {
    (1, "dfgd")
}