// Combining Smart Pointers
// Smart pointers ดูเหมือนจะมีข้อจำกัด แต่มันสามารถเอามารวมกันเพื่อให้ทำงานให้มีความหลากหลายเพิ่มขึ้นได้

// Rc<Vec<Foo>> - อนุญาตให้ทำการโคลน smart pointer หลายตัว ที่สามารถยืมเวกเตอร์เดียวกันของโครงสร้างข้อมูลที่ไม่เปลี่ยนรูปบน heap ได้

// Rc<RefCell<Foo>> - อนุญาตให้ smart pointer หลายตัวสามารถยืมโครงสร้าง Foo ตัวเดียวกัน โดยยืมได้ทั้งแบบ เปลี่ยนแปลงได้/เปลี่ยนแปลงไม่ได้

// Arc<Mutex<Foo>> - อนุญาตให้ smart pointer หลายตัวสามารถล็อกการยืม โดยยืมได้ทั้งแบบ เปลี่ยนแปลงได้/เปลี่ยนแปลงไม่ได้ ชั่วคราว​ ในลักษณะเฉพาะของเธรด CPU

// รายละเอียดหน่วยความจำ:

// คุณจะสังเกตเห็น รูปแบบการผสมผสานนี้ มาใช้กับข้อมูลประเภทที่แก้ไขไม่ได้ (ซึ่งเป็นไปได้ว่าจะถูกถือครองโดย smart pointer หลายตัว) เพื่อใช้แก้ไขข้อมูลภายในนั้น รูปแบบนี้ใน Rust เรียกว่า "interior mutability" สิ่งนี้เป็นรูปแบบที่เรานำมาใช้เลี่ยงกฎของการใช้หน่วยความจำในขณะ runtime โดยยังมีความปลอดภัยระดับเดียวกับการตรวจสอบของ Rust ในขณะ compile-time

use std::cell::RefCell;
use std::rc::Rc;

struct Pie {
    slices: u8,
}

impl Pie {
    fn eat_slice(&mut self, name: &str) {
        println!("{} took a slice!", name);
        self.slices -= 1;
    }
}

struct SeaCreature {
    name: String,
    pie: Rc<RefCell<Pie>>,
}

impl SeaCreature {
    fn eat(&self) {
        // use smart pointer to pie for a mutable borrow
        let mut p = self.pie.borrow_mut();
        // take a bite!
        p.eat_slice(&self.name);
    }
}

fn main() {
    let pie = Rc::new(RefCell::new(Pie { slices: 8 }));
    // ferris and sarah are given clones of smart pointer to pie
    let ferris = SeaCreature {
        name: String::from("ferris"),
        pie: pie.clone(),
    };
    let sarah = SeaCreature {
        name: String::from("sarah"),
        pie: pie.clone(),
    };
    ferris.eat();
    sarah.eat();

    let p = pie.borrow();
    println!("{} slices left", p.slices);
}

