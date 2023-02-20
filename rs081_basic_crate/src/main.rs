// Referencing Other Modules and Crates
// ของต่างๆในโมดูลสามารถอ้างถึงได้ด้วย โมดูลพาธแบบเต็ม std::f64::consts::PI.

// อีกวิธีที่ง่ายกว่าคือการใช้คีย์เวิร์ด use ซึ่งจะช่วยให้เราระบุสิ่งที่ต้องการใช้ จากโมดูลนั้น โดยไม่ต้องใส่ โมดูลพาธแบบเต็ม ตัวอย่างเช่นเมื่อเราใช้ use std::f64::consts::PI จะทำให้เราสามารถระบุเพียงแค่ PI ได้ในฟังก์ชันหลัก

// std คือ crate ของ standard library หรือไลบรารีมาตรฐานของ Rust ซึ่งเต็มไปด้วย โครงสร้างข้อมูล และฟังก์ชันที่มีประโยชน์ในการติดต่อกับระบบปฏิบัติการ

// สามารถค้นหา crates ได้จากแหล่งรวบรวมที่สร้างโดยชุมชนของเราที่ https://crates.io.

use std::f64::consts::PI;

fn main() {
    println!("Welcome to the playground!");
    println!("I would love a slice of {}!", PI);
}


// https://tourofrust.com/chapter_9_th.html