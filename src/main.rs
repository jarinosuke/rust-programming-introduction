use std::vec;

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

enum Event {
    Quit,
    KeyDown(u8),
    MouseDown { x: i32, y: i32 },
}