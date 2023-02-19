// การเรียกเมธอด
// ข้อที่แตกต่างจากฟังก์ชันก็คือ เมธอดนั้นเป็นฟังก์ชันที่ เป็นส่วนหนึ่งของ ชนิดตัวแปรสักตัวแบบเฉพาะเจาะจง

// static methods — คือเมธอดที่เป็นของชนิดตัวแปรเองเลย เรียกใช้ด้วยตัวดำเนินการ ::

// instance methods — คือเมธอดที่เป็นของตัวแปร เรียกใช้ด้วยตัวดำเนินการ .

// แล้วเราจะมาพูดเรื่องการสร้างเมธอดด้วยตัวเองกันอีกครั้งเร็วๆนี้

fn main() {
    // Using a static method to create an instance of String
    let s = String::from("Hello world!");
    // Using a method on the instance
    println!("{} is {} characters long.", s, s.len());
}

