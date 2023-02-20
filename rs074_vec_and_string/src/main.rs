// เพื่อนที่คุ้นเคย
// คราวนี้เรามาพิจารณา smart pointers ที่เราเคยเห็นมาแล้วเช่น Vec<T> และ String

// Vec<T> คือ smart pointer ที่ครอบครองพื้นที่ไบต์บางส่วนในหน่วยความจำ โดยที่คอมไพเลอร์ของ Rust ไม่รู้เลยว่า ไบต์พวกนั้นคืออะไร smart pointer จะแปลสิ่งนั้นว่ามันหมายถึงอะไรเพื่อจะได้หยิบมันมาจากหน่วยความจำ เพื่อมาจัดการด้วยการตามดูว่า โครงสร้างข้อมูลนั้นมีจุดเริ่มต้นและสิ้นสุดอยู่ที่ไหน จนพบ โครงสร้างข้อมูลตัวจริงที่ raw pointer ที่ชี้ไปถึง และนำมาจัดหน้าตาให้ดูดีใช้งานง่าย ตามอินเตอร์เฟสในที่สุด (ตัวอย่างเช่น my_vec[3])

// ในทำนองเดียวกัน String ก็จะตามดูไบต์ในหน่วยความจำ และใช้กระบวนการทางโปรแกรมบังคับให้สิ่งที่เขียนไว้ในนั้นอยู่ในรูปแบบ utf-8 ที่ถูกต้อง จะได้ dereference ของในหน่วยความจำนั้นผ่าน type &str

// โครงสร้างข้อมูลทั้งสองตัวนี้ใช้ unsafe ทำงานเพื่อเข้าไปจัดการกับ raw pointers

// รายละเอียดหน่วยความจำ:

// Rust มีของที่เทียบเคียงได้กับ malloc ของภาษา C ก็คือ alloc และ Layout เพื่อเอาหน่วยความจำของคุณออกมาจัดการ

use std::alloc::{alloc, Layout};
use std::ops::Deref;

struct Pie {
    secret_recipe: usize,
}

impl Pie {
    fn new() -> Self {
        // let's ask for 4 bytes
        let layout = Layout::from_size_align(4, 1).unwrap();

        unsafe {
            // allocate and save the memory location as a number
            let ptr = alloc(layout) as *mut u8;
            // use pointer math and write a few 
            // u8 values to memory
            ptr.write(86);
            ptr.add(1).write(14);
            ptr.add(2).write(73);
            ptr.add(3).write(64);

            Pie { secret_recipe: ptr as usize }
        }
    }
}
impl Deref for Pie {
    type Target = f32;
    fn deref(&self) -> &f32 {
        // interpret secret_recipe pointer as a f32 raw pointer
        let pointer = self.secret_recipe as *const f32;
        // dereference it into a return value &f32
        unsafe { &*pointer }
    }
}
fn main() {
    let p = Pie::new();
    // "make a pie" by dereferencing our 
    // Pie struct smart pointer
    println!("{:?}", *p);
}

