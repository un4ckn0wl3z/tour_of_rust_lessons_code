// loop
// อยากได้ลูปแบบไม่มีจุดจบเหรอ?

// Rust จัดให้

// break จะพาคุณหนีออกจากลูปเมื่อคุณต้องการ

// loop คำนี้มีความลับที่เดี๋ยวเราจะบอก

fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
        println!("x value: {}", x);
    }
    println!("final x value: {}", x);
}

