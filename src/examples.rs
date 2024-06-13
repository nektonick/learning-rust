#![allow(warnings, unused)]

pub fn examples() {
    println!("Hello, world!");
    println!("Sum: {}", sum(1.0, 2.0));

    let x: i128 = i128::MAX;
    println!("x: {x}");
    let xx: isize = isize::MAX;
    println!("xx: {xx}");

    let (a, b, c) = (100, 2000, 3000);
    interproduct(a, b, c);

    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);

    let _x = 20.0;
    let _y = 20;
    // assert_eq!(_x, _y);

    let n = 20;
    println!("fib({n}) = {}", fib(n));

    for x in 1..5 {
        println!("x: {x}");
    }

    for x in 1..=5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    let mut i = 0;
    let l = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };
    println!("l: {l}");

    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    println!("elements searched: {elements_searched}");

    let n = 4;
    println!("{n}! = {}", factorial(n));

    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");

    print_tuple((1, 2));

    let _array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);

    let mut point = (1, 2);
    let x_coord = &mut point.0;
    // let non_mutable_ref = &point.0;
    *x_coord = 20;
    println!("point: {point:?}");

    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4];
    // a[3] = -1;
    println!("s[0]: {}", s[0]);
    println!("s: {s:?}");
    println!("a: {a:?}");

    let s1: &str = "World";
    println!("s1: {s1}");
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");

    let f = format!("{s2}:{s1}");
    println!("{f}");
    let borrowed_string = &f[..];
    println!("{borrowed_string}");

    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);
    peter.age = 28;
    describe(&peter);
    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);
    let jackie = Person {
        name: String::from("Jackie"),
        ..avery
    };
    describe(&jackie);

    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    // let force = compute_thruster_force();
    // set_thruster_force(Newtons(force.0));
    // set_thruster_force(force);

    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);

    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);

    let input = 'X';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else"),
    }

    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i } => println!("y = 2, x = {i:?}"),
        Foo { y, .. } => println!("y = {y}, other fields were ignored"),
    }

    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));

    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
}

fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    // let s = if let Some(s) = maybe_string {
    //     s
    // } else {
    //     return Err(String::from("got None"));
    // };
    //
    // let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
    //     first_byte_char
    // } else {
    //     return Err(String::from("got empty string"));
    // };
    //
    // if let Some(digit) = first_byte_char.to_digit(16) {
    //     Ok(digit)
    // } else {
    //     Err(String::from("not a hex digit"))
    // }

    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}

/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
            let left = match eval(*left) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            let right = match eval(*right) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            Ok(match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => {
                    if right == 0 {
                        return Err(String::from("division by zero"));
                    } else {
                        left / right
                    }
                }
            })
        }
        Expression::Value(v) => Ok(v),
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

#[repr(u32)]
enum Bar {
    A, // 0
    B = 10000,
    C, // 10001
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

struct PoundsOfForce(f64);

struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

struct Point(i32, i32);

struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn print_tuple((left, right): (i32, i32)) {
    println!("left: {left}, right: {right}");
}

fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b * c
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn sum(x: f32, y: f32) -> f32 {
    x + y
}

#[test]
/// Test the sum function with different inputs and expected results.
/// Asserts that the sum function returns the correct result for each input.
fn test_sum() {
    // Test case 1: sum(1.0, 2.0) should equal 3.0.
    assert_eq!(sum(1.0, 2.0), 3.0);

    // Test case 2: sum(0.0, 0.0) should equal 0.0.
    assert_eq!(sum(0.0, 0.0), 0.0);

    // Test case 3: sum(1.5, -2.5) should equal -1.0.
    assert_eq!(sum(1.5, -2.5), -1.0);

    // Test case 4: sum(-1.5, 2.5) should equal 1.0.
    assert_eq!(sum(-1.5, 2.5), 1.0);

    // Test case 5: sum(-1.5, -2.5) should equal -4.0.
    assert_eq!(sum(-1.5, -2.5), -4.0);

    // Test case 6: sum(1.5, 2.5) should equal 4.0.
    assert_eq!(sum(1.5, 2.5), 4.0);
}
