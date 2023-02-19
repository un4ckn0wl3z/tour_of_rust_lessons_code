// Enumerations
// Enumerations ช่วยให้คุณสามารถสร้าง type ใหม่ ที่สามารถมีค่าขององค์ประกอบที่ติดแท็กหลายรายการโดยใช้คีย์เวิร์ด enum

// match จะเป็นตัวช่วยให้มั่นใจได้ว่าจะจัดการ enum ทุกตัวได้ครบ ว้าว สุดยอดเครื่องมือ ที่ทำให้เราได้โค้ดที่มีคุณภาพ เยี่ยมไปเลย


#![allow(dead_code)] // this line prevents compiler warnings

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is a crab",ferris.name),
        Species::Octopus => println!("{} is a octopus",ferris.name),
        Species::Fish => println!("{} is a fish",ferris.name),
        Species::Clam => println!("{} is a clam",ferris.name),
    }
}
