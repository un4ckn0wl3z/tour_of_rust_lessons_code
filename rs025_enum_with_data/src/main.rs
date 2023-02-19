// Enumerations With Data
// ในองค์ประกอบ enum ยังอนุญาตให้สามารถมีประเภทข้อมูล ได้อย่างน้อยหนึ่งชนิดอีกด้วย ซึ่งจะมีพฤติกรรมคล้ายกับ union ในภาษาซี

// เมื่อใช้ match มาจับคู่ของใน enum คุณสามารถผูกชื่อตัวแปรให้แต่ละค่านั้นได้เลย

// รายละเอียดหน่วยความจำของ enum:

// ค่าข้อมูลของ enum จะมีขนาดในหน่วยความจำ เท่ากับขนาดขององค์ประกอบสมาชิกตัวที่ใหญ่ที่สุด นั่นทำให้ไม่ว่าจะเป็นค่าใดก็จะสามารถวางลงบนพื้นที่ในหน่วยความจำได้แน่นอน
// นอกจากนี้แล้ว ประเภทข้อมูลของสมาชิก (ถ้ามี) แต่ละตัวจะมีเลขประจำตัวแทนแต่ละแท็กด้วย
// รายละเอียดอื่นๆ:

// enum ใน Rust ก็คือสิ่งที่หลายๆคนรู้จักกันในชื่อ tagged union
// เวลาที่ใครๆพูดกันว่า Rust มี algebraic types เขากำลังหมายถึงการเอา type หลายๆตัวมารวมกันสร้างเป็น type ใหม่นั่นเอง

#![allow(dead_code)] // this line prevents compiler warnings

enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal"),
    }
}
