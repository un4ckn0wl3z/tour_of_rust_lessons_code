// Structures
// struct คือกลุ่มของฟิลด์

// ฟิลด์ ก็คือข้อมูลธรรมดานี่แหล่ะ ที่รวมอยู่ในโครงสร้าง ซึ่งมันเป็นได้ตั้งแต่ ชนิดตัวแปรขึ้นพื้นฐานที่สุด หรืออาจจะเป็นโครงสร้างอีกตัวซ้อนกันก็ได้

// การนิยามสิ่งนี้ ก็เหมือนกับพิมพ์เขียวให้คอมไฟเลอร์รู้ว่าฟิล์ดพวกนี้จะวางอยู่ติดกันในหน่วยความจำ

#[allow(dead_code)]
struct SeaCreature {
    // String is a struct
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    println!("Hello, world!");
}
