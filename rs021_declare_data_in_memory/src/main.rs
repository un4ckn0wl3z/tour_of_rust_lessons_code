// การสร้างข้อมูลในหน่วยความจำ
// เมื่อเราสร้าง อินสแตนซ์ จาก struct สักตัวในโค้ดของเรา โปรแกรมของเราจะสร้าง ข้อมูลจากโครงสร้างฟิลด์ไว้เคียงข้างกันในหน่วยความจำ

// เราสร้างอินสแตนซ์จากฟิลด์ทั้งหมดภายใน

// StructName { ... }.

// การเข้าถึงฟิลด์ใน Struct ทำได้ด้วยการใช้ตัวดำเนินการ .

// รายละเอียดหน่วยความจำจากตัวอย่างนี้:

// ข้อความในเครื่องหมายคำพูด เป็นข้อมูลที่ใช้อ่านอย่างเดียว (เช่น "Ferris"), พวกนี้จะไปอยู่ใน data memory region
// มีการเรียกฟังก์ชัน String::from เพื่อสร้าง struct ของ String เพื่อนำไปวางเคียงข้างกัน กับฟิลด์ใน SeaCreature ใน stack โดยที่ String คือข้อความที่สามารถเปลี่ยนได้และทำได้โดย:
// ไปสร้างหน่วยความจำใน heap สำหรับข้อความที่ต้องการแก้ไข
// จัดเก็บสิ่งที่จะอ้างถึงหน่วยความจำนั้นไว้ใน heap แล้วค่อยเอาสิ่งนี้ ไปไว้ใน struct ของ String (มีเพิ่มเติมเรื่องนี้ในบทเรียนต่อๆไป)
// ในที่สุดเพื่อนของเราทั้งสอง Ferris และ Sarah จึงมีโครงสร้างข้อมูลของตัวเอง แถมยังมีตำแหน่งที่ชัดเจน และคงที่ด้วย อยู่ในโปรแกรมของเรา และแน่นอนว่านั่นอยู่ใน stack

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none"),
    };
    
    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
}

