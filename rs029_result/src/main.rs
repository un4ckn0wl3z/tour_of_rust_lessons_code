// Result
// Rust มี generic enum ในตัว มีชื่อว่า Result มาช่วยให้เราสามารถคืนค่าที่มีความเป็นไปได้ที่จะล้มเหลว นี่เป็นสำนวนทางภาษาของเรา ในการจัดการ error

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// สังเกตดูว่า generic type ของเรามี parameterized types อยู่หลายตัว และแต่ละตัวคั่นด้วยลูกน้ำ

// enum นี้เป็นของส่วนรวม ทำให้อินสแตนซ์ของ enum ทุกตัว สามารถมีค่าเป็น Ok และ Err ได้เลย

fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))   
    }
}

fn main() {
    let result = do_something_that_might_fail(12);

    // match lets us deconstruct Result elegantly and ensure we handle all cases!
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}",e),
    }
}

