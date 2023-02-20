// การสร้างเมธอดไว้ใน Traits
// ตัว Traits เองสามารถมีเมธอดที่ทำงานจริงได้ด้วย

// แต่ฟังก์ชันพวกนี้จะไม่มีการเข้าถึงฟิลด์ใน struct ตรงๆ แต่จะได้ประโยชน์ในแง่ ที่มันเห็นพฤติกรรมที่ trait มีไว้ใช้ทั้งหมด

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
    
    fn make_alot_of_noise(&self){
        self.make_noise();
        self.make_noise();
        self.make_noise();
    }
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
    creature.make_alot_of_noise();
}

