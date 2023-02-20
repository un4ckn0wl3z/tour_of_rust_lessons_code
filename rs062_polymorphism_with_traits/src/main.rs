// Polymorphism ด้วย Traits
// Rust สนับสนุน polymorphism ด้วย traits โดย Traits จะช่วยให้เราเชื่อมโยงกลุ่มของเมธอด เข้ากับ struct type ได้

// ขั้นแรกเราต้องกำหนดลักษณะเฉพาะของเมธอดใน trait ก่อน:

// trait MyTrait {
//     fn foo(&self);
//     ...
// }
// เมื่อมี struct ใดต้องการอิมพลีเมนต์ trait มันจะสร้างสัญญาขึ้นมาเพื่ออนุญาต ให้เราใช้วิธีการโต้ตอบกับ struct แบบที่ trait กำหนดไว้ (จากตัวอย่าง &dyn MyTrait)

// โดยไม่ต้องรู้เลยว่า type ที่แท้จริงของมันคืออะไร

// วิธีการทำให้ struct ใช้เมธอดของ traits ทำภายในบล็อกแบบนี้:

// impl MyTrait for MyStruct { 
//     fn foo(&self) {
//         ...
//     }
//     ... 
// }

struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_noise();
}

