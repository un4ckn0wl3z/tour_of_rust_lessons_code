// Generic Function แบบย่อ
// Rust ย่อรูปแบบการประกาศ generics ที่กำหนด trait ได้แบบนี้:

// fn my_function(foo: impl Foo) {
//     ...
// }
// ซึ่งจะเหมือนกับการเขียแบบนี้:

// fn my_function<T>(foo: T)
// where
//     T:Foo
// {
//     ...
// }


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

fn generic_make_noise(creature: &impl NoiseMaker)
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

