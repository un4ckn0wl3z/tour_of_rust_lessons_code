// String Slice
// string slice ใดก็ตามคือการอ้างถึง ไบต์กลุ่มหนึ่งในหน่วยความจำที่อยู่ในรูป utf-8 ที่ถูกต้องเสมอ

// string slice (ส่วนย่อย) แต่ละชิ้นของ str ก็ต้องเป็น utf-8 ที่ถูกต้องแน่นอนเช่นกัน

// เมธอดที่ใช้โดยทั่วไปของ &str ได้แก่:

// len จะคือค่าความยาวของข้อความ string ในขนาดของไบต์ (ไม่ใช่ขนาดของตัวอักขระ)
// starts_with/ends_with ใช้ทดสอบแบบง่ายๆ
// is_empty คือค่า true ถ้าความยาวเป็นศูนย์
// find คืน Option<usize> ที่เป็นตำแหน่งแรกของคำที่ต้องการค้นหา


fn main() {
    let a = "hi 🦀";
    println!("{}", a.len());
    let first_word = &a[0..2];
    let second_word = &a[3..7];
    // let half_crab = &a[3..5]; FAILS
    // Rust does not accept slices of invalid unicode characters
    println!("{} {}", first_word, second_word);
}
