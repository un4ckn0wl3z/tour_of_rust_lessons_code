// while
// while ให้คุณใส่เงื่อนไขลงไปในลูปได้อย่างง่ายดาย

// ถ้าเงื่อนไขถูกประเมินว่าเป็น false มันจะออกจากลูป

fn main() {
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("final x value: {}", x);
}

