// กลับมาที่ Main ที่ผิดพลาดได้ อีกครั้ง
// โค้ดของ Rust มีวิธีการมากมายในการแสดงค่า error ซึ่งในไลบรารีมาตรฐานก็มี trait กลาง ชื่อ std::error::Error เพื่อแสดงค่า error อยู่ด้วย

// เราสามารถใช้ smart pointer Box ด้วย type Box<dyn std::error::Error> มาเป็น type ส่วนกลางในการคืนค่า error ได้ เพราะมันยอมให้เราส่ง error ใน heap ขึ้นไปเรื่อยๆ เพื่อให้เราจัดการมันในระดับที่สูงขึ้นไปได้ โดยไม่ต้องรู้ว่า type จริงๆมันคืออะไร

// เมื่อตอนต้นของ Tour of Rust เราได้เรียนรู้ไปว่า main สามารถคืนค่า error ได้ ตอนนี้ เราจะได้คืน type ที่ครอบคลุม error ได้เกือบทุกแบบที่อาจเกิดขึ้นในโปรแกรม ตราบใดที่ error นั้นอิมพลีเมนต์ trait Error กลาง ของ Rust

// fn main() -> Result<(), Box<dyn std::error:Error>>

use core::fmt::Display;
use std::error::Error;

struct Pie;

#[derive(Debug)]
struct NotFreshError;

impl Display for NotFreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This pie is not fresh!")
    }
}

impl Error for NotFreshError {}

impl Pie {
    fn eat(&self) -> Result<(), Box<dyn Error>> {
        Err(Box::new(NotFreshError))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let heap_pie = Box::new(Pie);
    heap_pie.eat()?;
    Ok(())
}

