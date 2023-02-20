// Smart Pointers
// นอกเหนือจากความสามารถในการสร้างตัวอ้างอิงให้กับข้อมูลของ type ที่มีอยู่จริง ด้วยการใช้ตัวดำเนินการ & แล้ว Rust มีอีกความสามารถหนึ่งนั่นคือการสร้าง โครงสร้าง reference-like (การอ้างอิงเสมือน) ที่มีชื่อเรียกว่า smart pointers

// การอ้างอิง ในมุมมองที่สูงขึ้นไปอีก มันคือการเข้าถึง type หนึ่งได้จากอีก type หนึ่ง โดยที่ Smart pointer จะมีพฤติกรรมต่างไปจาก การอ้างอิงปกติ ตรงที่มันจะทำงาน ตามที่โปรแกรมเมอร์เขียนตรรกะให้นั่นเพราะว่า โปรแกรมเมอร์ - อย่างคุณที่แหล่ะ - คือสิ่งที่ ฉลาด ที่สุด

// โดยทั่วไป smart pointers จะอิมพลีเมนต์ Trait เหล่านี้ Deref, DerefMut, และ Drop เพื่อกำหนดทิศทางให้ ตรรกะว่าจะเกิดอะไรขึ้นเมื่อ struct พยายาม dereference ด้วยตัวดำเนินการ * และ .


use std::ops::Deref;

struct TattleTell<T> {
    value: T,
}

impl<T> Deref for TattleTell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("{} was used!", std::any::type_name::<T>());
        &self.value
    }
}
fn main() {
    let foo = TattleTell {
        value: "secret message",
    };
    // dereference occurs here immediately 
    // after foo is auto-referenced for the
    // function `len`
    println!("{}", foo.len());
}
