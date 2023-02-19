// Escaping Characters
// มีอักขระหลายตัวที่ไม่สามารถเห็นได้ด้วยตา escape codes จึงเข้ามาช่วยในเรื่องนี้

// Rust รับรอง escape codes ที่ภาษา C ใช้กันอยู่แล้ว:

// \n - newline
// \r - carriage return
// \t - tab
// \\ - backslash
// \0 - null
// \' - single-quote
// ดูทั้งหมดได้ ที่นี่ https://doc.rust-lang.org/reference/tokens.html.

fn main() {
    let a: &'static str = "Ferris says:\t\"hello\"";
    println!("{}",a);
}
