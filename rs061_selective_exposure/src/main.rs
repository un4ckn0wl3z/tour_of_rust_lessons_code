// Abstraction ด้วยการทำ Selective Exposure
// Rust สามารถซ่อนการทำงานภายใน objects ได้

// เพราะโดยปกติแล้วการเข้าถึง ฟิดล์และเมธอด จะทำได้เฉพาะในโมดูลเดียวกันเท่านั้น

// ถ้าอยากจะเปิดเผย ฟิลด์และเมธอดของ struct ออกสู่ภายนอกโมดูลให้ใช้คีย์เวิร์ด pub

struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}

