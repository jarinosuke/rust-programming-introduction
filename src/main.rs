//use std::{fmt::rt::v1::Count, vec, writeln};
use std::io::Write;
use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use async_trait::async_trait;

mod module_a;

fn main() {
    println!("Hello, world!");

       // 文字列
       let s1: String = String::from("Hello, World");
       let s2: &str = &s1;
       let s3: String = s2.to_string();
   
       // タプル
       let mut t = (1, "2");
       t.0 = 2;
       t.1 = "3";

       // 配列

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];

    println!("{:?}", &a[1..3]);

    // Class
    let p = Person {
        name: String::from("John"),
        age: 8,
    };

    // Enum
    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 10 };

    // Result 
    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }

    // if let Ok(code) = result {
    //     println!("code: {}", code);
    // }
    // println!("code: {}", result.unwrap_or(-1));
    let result2: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result2.unwrap_or(-1));
    
    let result3: Result<i32, String> = Ok(200);
    let next_result3 = result3.and_then(func);
    let result4: Result<i32, String> = Err("error".to_string());
    let next_result4 = result4.and_then(func);

    // Vec
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    println!("{}", v1[0]);
    for element in &v1 {
        println!("{}", element);
    }

    // Box
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));

    // if
    let number = 1;
    if 0 < number {
        println!("0 < number");
    }  else if number < 0 {
        println!("number < 0");
    } else {
        println!("number == 0");
    }
    let number2 = 1;
    let result = if 0 <= number2 {
        number2
    } else {
        -number2
    };

    // Loop
    let mut count = 0;
    let result2 = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    // While
    let mut count2 = 0;
    while count2 < 10 {
        println!("count: {}", count2);
        count2 += 1;
    }

    // For
    let count3 = 0;
    for count3 in 0..10 {
        println!("count: {}", count3);
    }
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for element in &array {
        println!("element: {}", element);
    }

    // Label
    'main: loop {
        println!("start main loop");
        'sub: loop {
            println!("start sub loop");
            break 'main;
        }
    }

    // Match
    let i = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"),
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    let result5: Result<i32, String> = Ok(100);
    let result_number = match result5 {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        },
    };

    // Range
    for number in 1..5 {
        println!("{}", number);
    }

    // Iterator
    let it = Iter{
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }

    //Impl
    let p2 = Person {
        name: String::from("Taro"),
        age: 20,
    };

    p2.say_name();
    p2.say_age();
    p2.say_name().say_age();

    //Macro
    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, " is 123");
    dbg!(w);

    //Future
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }

    executor::block_on(something_great_async_function());
}

fn func(code: i32) -> Result<i32, String> {
    println!("code {}", code);
    Ok(100)
}

fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?;
    println!("code: {}", code);
    Ok(100)
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    fn say_name(&self) -> &Self {
        println!("I am {}", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        self
    }
}

enum Event {
    Quit,
    KeyDown(u8),
    MouseDown { x: i32, y: i32 },
}

enum Color {
    Red,
    Blue,
    Green,
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}
// Future
struct CountDown(u32);

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 == 0 {
            Poll::Ready("Zero!!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans1 = async_add(2, 3).await;
    let ans2 = async_add(3, 4).await;
    let ans3 = async_add(4, 5).await;
    let result = ans1 + ans2 + ans3;
    println!("{}", result);
    result 
}

fn something_great_async_function2() -> impl Future<Output = i32> {
    async {
        let ans = async_add(3, 2).await;
        println!("{}", ans);
        ans
    }
}

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();
    async move {
        println!("{}", outside_variable);
    }
}

#[async_trait]
trait AsyncTrait {
    async fn f() {
        println!("test");
    }
}