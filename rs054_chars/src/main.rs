// Chars
// การทำงานกับ Unicode นั้นค่อนข้างยุ่งยากมาก Rust จึงเสนอวิธีการดึงข้อมูล ไบต์ของ utf-8 ออกมาเป็นเวกเตอร์ของตัวแปรแบบ char

// char แต่ละตัวจะยาว 4ไบต์เสมอ (ซึ่งจะช่วยให้การหาค่าแต่ละตัวทำได้อย่างมีประสิทธิภาพ)


fn main() {
    // collect the characters as a vector of char
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // should be 4
    // since chars are 4 bytes we can convert to u32
    println!("{}", chars[3] as u32);
}

