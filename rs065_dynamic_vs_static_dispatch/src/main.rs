// Dynamic vs Static Dispatch
// เมธอดถูกเรียกได้จากสองวิธี:

// static dispatch - เมื่อรู้ type ของอินสแตนซ์เราก็รู้อยู่แล้วว่ามันจะเรียกใช้อย่างไร
// dynamic dispatch -เมื่อเราไม่รู้ว่าอินสแตนซ์นั้นเป็น type อะไรกันแน่ เราจึงต้อง หาวิธีที่รู้ให้ได้ว่าจะเรียกใช้ฟังก์ชันอะไรถึงจะถูกต้อง
// &dyn MyTrait เป็นประเภทของ trait ที่จะทำให้เราทำงานกับอินสแตนซ์นั้นได้ โดยทางอ้อม ผ่านทาง dynamic dispatch

// เมื่อจะใช้ dynamic dispatch เมื่อไหร่ Rust แนะนำให้ใส่ dyn เข้าไปหน้าตัวแปร trait เพื่อให้คนอื่นรับรู้

// รายละเอียดหน่วยความจำ:

// Dynamic dispatch นั้นจะทำงานได้ช้า เพราะว่า ต้องเสียเวลาค้นหาฟังก์ชันตัวจริงมาใช้งาน

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

fn static_make_noise(creature: &SeaCreature) {
    // we know the real type
    creature.make_noise();
}

fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
    // we don't know the real type
    noise_maker.make_noise();
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    static_make_noise(&creature);
    dynamic_make_noise(&creature);
}
