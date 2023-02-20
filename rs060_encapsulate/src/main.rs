// Encapsulation ด้วยการมีเมธอด
// Rust สนับสนุนแนวคิดของ object ด้วยการผูกฟังก์ชันบางตัวไว้กับ struct ได้ (เราเรียกมันว่า เมธอด แทน)

// โดยที่พารามิเตอร์ตัวแรกของเมธอดจะต้องอ้างถึง อินสแตนซ์ ที่มันยึดโยงอยู่ เพื่อให้เรียกใช้เป็นเมธอดได้ (ตัวอย่างเช่น instanceOfObj.foo()) วิธีที่ Rust ทำก็คือ

// &self - ใช้อ้างถึงอินสแตนซ์แบบเปลี่ยนแปลงค่าไม่ได้
// &mut self - ใช้อ้างถึงอินสแตนซ์แบบเปลี่ยนแปลงค่าได้
// การกำหนดการทำงานของมันจะทำภายในบล็อกของคีย์เวิร์ด impl:

// impl MyStruct { 
//     ...
//     fn foo(&self) {
//         ...
//     }
// }


struct SeaCreature {
    noise: String,
}

impl SeaCreature {
    fn get_sound(&self) -> &str {
        &self.noise
    }
}

fn main() {
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}

