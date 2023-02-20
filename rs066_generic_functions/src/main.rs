// Generic Functions
// Generics ใน Rust ทำงานประสานกับ traits เมื่อเราประกาศพารามิเตอร์ type T เราสามารถระบุว่า type ไหนที่สามารถใช้เป็นอาร์กิวเมนต์ได้บ้าง ด้วยการลิสต์รายการ ว่าเราอยากได้อาร์กิวเมนต์ที่อิมพลีเมนต์ traits ใดบ้าง

// จากตัวอย่างนี้ type T ต้องอิมพลีเมนต์ ตาม trait Foo

// fn my_function<T>(foo: T)
// where
//     T:Foo
// {
//     ...
// }
// ด้วยการใช้ generics ทำให้เราสามารถสร้าง static typed functions ได้ตอน compile time เราจึงสามารถรู้ type และขนาดของมัน และสามารถทำ static dispatch และเก็บ ขนาดของมันได้



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

fn generic_make_noise<T>(creature: &T)
where
    T: NoiseMaker,
{
    // we know the real type at compile-time
    creature.make_noise();
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    generic_make_noise(&creature);
}

