// Functions
// ฟังก์ชัน มีพารามิเตอร์ได้ศูนย์ตัวหรือมากกว่านั้น

// ในตัวอย่างนี้ ฟังก์ชัน add รับอากิวเมนต์ประเภท i32 (signed integer of 32-bit length)

// ชื่อฟังก์ชันจะเป็น snake_case เสมอ

fn add(x: i32, y: i32) -> i32{
    return x + y;
}

fn main() {
    println!("{}", add(42, 13))
}
