// Sharing Across Threads
// Mutex เป็นโครงสร้างข้อมูลแบบคอนเทนเนอร์ ที่ถูกถือไว้โดย smart pointer โดยมันจะรับข้อมูลเข้ามา และยอมให้เรายืม การอ้างอิง ข้อมูลที่อยู่ในนั้น ทั้งแบบยืมโดยไม่เปลี่ยนแปลงอะไร และยืมโดยยอมให้เปลี่ยนแปลงได้ โดยป้องกันการยืมไปใช้ในทางที่ผิดด้วยการให้ระบบปฏิบัติการ จำกัดการเข้าถึงได้เพียง CPU เดียวในช่วงเวลาหนึ่ง และกันไม่ให้เธรดอื่นเข้าถึงได้จนกว่าเธรดเดิมจะทำงานเสร็จแล้วค่อยปลดล็อกการยืม

// มัลติเธรดอยู่นอกเหนือขอบเขตของ Tour of Rust แต่ Mutex เป็นส่วนพื้นฐานของการควบคุม เธรดหลายตัวของ CPU ในการเข้าถึงข้อมูลเดียวกัน

// มี smart pointer พิเศษอีกตัวที่เหมือนกับ Rc แต่ต่างกันที่ มันใช้เพิ่มจำนวนตัวอ้างอิงแบบ thread-safe ซึ่งจะใช้บ่อยเมื่อมีการอ้างถึงหลายๆตัวไปที่ Mutex เดียวกัน

use std::sync::Mutex;

struct Pie;

impl Pie {
    fn eat(&self) {
        println!("only I eat the pie right now!");
    }
}

fn main() {
    let mutex_pie = Mutex::new(Pie);
    // let's borrow a locked immutable reference of pie
    // we have to unwrap the result of a lock
    // because it might fail
    let ref_pie = mutex_pie.lock().unwrap();
    ref_pie.eat();
    // locked reference drops here, and mutex protected value can be used by someone else
}

