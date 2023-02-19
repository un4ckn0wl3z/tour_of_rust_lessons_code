// Tuple-like Structs
// เพื่อความสะดวกคุณสามารถสร้างโครงสร้างที่หน้าตาเหมือนทูเพิลได้เลยด้วย

struct Location(i32, i32);

fn main() {
    // This is still a struct on a stack
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}

