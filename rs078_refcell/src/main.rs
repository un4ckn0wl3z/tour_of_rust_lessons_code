// Sharing Access
// RefCell เป็นโครงสร้างข้อมูลแบบคอนเทนเนอร์ ที่ถูกถือไว้โดย smart pointer โดยมันจะรับข้อมูลเข้ามา และยอมให้เรายืม การอ้างอิง ของที่อยู่ในนั้นไม่ว่าจะเป็นอะไร ทั้งแบบยืมโดยไม่เปลี่ยนแปลงอะไร และยืมโดยยอมให้เปลี่ยนแปลงได้ และมีการป้องกันการยืมไม่ให้ถูกละเมิด ด้วยการบังคับใช้กฎรักษาความปลอดภัยในหน่วยความจำของ Rust ตอน runtime เมื่อคุณขอยืมข้อมูล:

// การยืมเลือกได้เพียงอย่างเดียว คือ ยืมแบบเปลี่ยนแปลงค่าได้ หนึ่งตัว หรือยืมแบบเปลี่ยนค่าไม่ได้ หลายตัว จะทำทั้งสอบแบบในเวลาเดียวกันไม่ได้

// หากคุณละเมิดกฎของ RefCall จะทำให้เกิด panic

use std::cell::RefCell;

struct Pie {
    slices: u8
}

impl Pie {
    fn eat(&mut self) {
        println!("tastes better on the heap!");
        self.slices -= 1;
    }
}

fn main() {
    // RefCell validates memory safety at runtime
    // notice: pie_cell is not mut!
    let pie_cell = RefCell::new(Pie{slices:8});
    
    {
        // but we can borrow mutable references!
        let mut mut_ref_pie = pie_cell.borrow_mut();
        mut_ref_pie.eat();
        mut_ref_pie.eat();
        
        // mut_ref_pie is dropped at end of scope
    }
    
    // now we can borrow immutably once our mutable reference drops
     let ref_pie = pie_cell.borrow();
     println!("{} slices left",ref_pie.slices);
}

