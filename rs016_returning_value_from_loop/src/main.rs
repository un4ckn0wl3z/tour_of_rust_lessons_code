// Returning Values From loop
// loop สามารถใช้ break เพื่อคืนค่าได้ด้วย

fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);
}
