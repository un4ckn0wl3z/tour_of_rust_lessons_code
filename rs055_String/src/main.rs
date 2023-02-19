// String
// String ก็คือ struct ที่ครอบครองลำดับของ ไบต์ของ utf-8 ที่อยู่ใน heap

// และเพราะว่ามันอยู่ใน heap มันจึงสามารถขยายขนาด แก้ไขค่า หรืออื่นๆ แบบที่ ข้อความสตริงปกติทำไม่ได้

// เมธอดที่ใช้ทั่วไปได้แก่:

// push_str ใช้เพิ่ม utf-8 ต่อท้ายสตริง
// replace ใช้แทนลำดับคำในรูปแบบ utf-8 ด้วยค่าอื่น
// to_lowercase/to_uppercase ใช้เปลี่ยนลักษณะตัวพิมพ์เล็กหรือใหญ่
// trim ใช้ตัดแต่งช่องว่าง
// เมื่อ String ถูก drop หน่วยความจำของมันที่อยู่ใน heap ก็จะ drop ตามไปด้วย

// String มีตัวดำเนินการ + ไว้ใช้ต่อค่ากับ &str และคืนตัวมันเองมาให้ แต่มันอาจจะดูแปลกไปสักน้อยเวลาใช้งาน

fn main() {
    let mut helloworld = String::from("hello");
    helloworld.push_str(" world");
    helloworld = helloworld + "!";
    println!("{}", helloworld);
}
