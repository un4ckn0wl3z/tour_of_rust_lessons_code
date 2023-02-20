// Referencing Counting
// Rc เป็น smart pointer ที่ย้ายข้อมูลจาก stack ไปอยู่ใน heap มันช่วยให้เราโคลน smart pointer Rc ตัวอื่นๆ โดยทุกตัวจะสามารถ ยืมโดยไม่เปลี่ยนแปลง ข้อมูลที่อยู่ใน heap ได้

// เมื่อ smart pointer ตัวสุดท้ายถูก drop เท่านั้นจึงจะคืนหน่วยความจำใน heap

use std::rc::Rc;

struct Pie;

impl Pie {
    fn eat(&self) {
        println!("tastes better on the heap!")
    }
}

fn main() {
    let heap_pie = Rc::new(Pie);
    let heap_pie2 = heap_pie.clone();
    let heap_pie3 = heap_pie2.clone();

    heap_pie3.eat();
    heap_pie2.eat();
    heap_pie.eat();

    // all reference count smart pointers are dropped now
    // the heap data Pie finally deallocates
}
