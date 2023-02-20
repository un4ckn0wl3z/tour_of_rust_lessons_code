// Box
// Box เป็นโครงสร้างข้อมูลที่ยอมให้เราย้ายข้อมูลจาก stack ไปไว้ใน heap ได้

// Box คือโครงสร้างที่รู้จักกันในอีกชื่อว่า smart pointer เพราะว่ามันครอบครองพอยเตอร์ ที่ชี้ไปยังข้อมูลของเราใน heap

// และด้วยเหตุที่ Box เป็น struct ที่เรารู้ขนาดแน่นอน (เพราะว่ามันแค่ถือครองพอยเตอร์) มันจึงเป็นตัวเลือกที่ถูกนำมาใช้บ่อยๆ เพื่อเก็บการอ้างอิงไปยังของชิ้นอื่นใน struct เมื่อต้องการขนาดของฟิลด์ที่แน่นอน

// โดยปกติ Box สามารถใช้ได้จากทุกที่:

// Box::new(Foo { ... })


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

struct Ocean {
    animals: Vec<Box<dyn NoiseMaker>>,
}

fn main() {
    let ferris = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    let sarah = SeaCreature {
        name: String::from("Sarah"),
        noise: String::from("swish"),
    };
    let ocean = Ocean {
        animals: vec![Box::new(ferris), Box::new(sarah)],
    };
    for a in ocean.animals.iter() {
        a.make_noise();
    }
}

