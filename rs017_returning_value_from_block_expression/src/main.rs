// การคืนค่าจาก Block Expressions
// if, match, ฟังก์ชัน, และ กลุ่มคำสั่งในบล็อก ใน Rust ล้วนมีวิธีการคืนค่าที่ดูน่าสนใจ

// หากว่าในคำสั่งสุดท้ายที่อยู่ในบล็อกของ if, match, ฟังก์ชัน หรือ กลุ่มคำสั่งในบล็อก เป็นคำสั่งที่ไม่มี ; Rust จะส่งค่าในบรรทัดนั้นกลับไปเข้าตัวแปรได้เลย ซึ่งนี่มันทำให้ตรรกะที่เขียนดูกระชับและงดงามมาก

// สังเกตดูจะพบว่า มันยอมให้ if เขียนอยู่ในรูปแบบ ternary expression ในบรรทัดเดียวได้ด้วย

fn example() -> i32 {
    let x = 42;
    // Rust's ternary expression
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hotdog";
    let result = match food {
        "hotdog" => "is hotdog",
        // notice the braces are optional when its just a single return expression
        _ => "is not hotdog",
    };
    println!("identifying food: {}", result);

    let v = {
        // This scope block lets us get a result without polluting function scope
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    // The idiomatic way to return a value in rust from a function at the end
    v + 4
}

fn main() {
    println!("from function: {}", example());
}
