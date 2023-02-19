// Vectors
// generic type ที่มีประโยชน์ที่สุดประเภทหนึ่งคือพวกที่เป็นคอลเล็คชัน vector คือหนึ่งในนั้น มันเป็นลิสต์ที่มีขนาดแปรผันได้ มาในรูปของ struct Vec

// มาโคร vec! ใช้สร้าง vector แบบง่ายๆ แทนที่จะไปสร้างเองด้วยมือ

// Vec มาพร้อมเมธอด iter() ใช้สร้างตัววนซ้ำเวลาเอาไปใส่ใน for ลูป

// รายละเอียดหน่วยความจำ:

// Vec เป็น struct แต่มันแค่ใช้เก็บการอ้างอิงไปยัง ลิสต์ตัวหนึ่งใน heap
// vector เริ่มต้นด้วยความจุขนาดหนึ่ง เมื่อมีการเพิ่มของจนล้นความจุนั้น มันจะไป จัดสรรเนื้อที่ใหม่บน heap เพื่อเก็บลิสต์ใหม่ ด้วยความจุที่ใหญ่ขึ้น

fn main() {
    // We can be explicit with type
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // But look how clever Rust is about determining the type automatically
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // That's a beautiful macro!
    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}
