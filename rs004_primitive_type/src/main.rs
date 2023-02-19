// Types พื้นฐาน
// Rust มี type ที่เราคุ้นเคยอยู่หลายตัว:

// booleans - bool สำหรับค่า true/false
// unsigned integers - u8 u16 u32 u64 u128 สำหรับตัวเลขทั้งหมดที่มีค่าเป็นบวก
// signed integers - i8 i16 i32 i64 i128 สำหรับตัวเลขทั้งหมด
// pointer sized integers - usize isize สำหรับค่าดัชนี(index) และขนาดของ ของในหน่วยความจำ
// floating point - f32 f64
// tuple - (value, value, ...) สำหรับส่งของตามลำดับบน stack
// arrays - [value, value, ...] กลุ่มข้อมูลประเภทเดียวกันที่รู้ขนาดที่แน่นอนตั้งแต่ compile time
// slices - กลุ่มข้อมูลประเภทเดียวกันที่รู้ขนาดที่แน่นอนเมื่อตอน runtime
// str(string slice) - ข้อความ ที่รู้ขนาดที่แน่นอนเมื่อตอน runtime
// เรื่องประเภทข้อมูลแบบข้อความ อาจจะมีความซับซ้อนมากกว่าที่คุณเคยรู้ในภาษาอื่น; เนื่องจากว่า Rust เป็นภาษา system programming language จึงให้ความสำคัญกับปัญหาในเรื่องหน่วยความจำที่คุณอาจจะยังไม่คุ้นเคย ซึ่งเราจะลงในรายละเอียดเพิ่มเติมกันภายหลัง

// ตัวแปรประเภทตัวเลขทั้งหลาย สามารถระบุความชัดเจนให้มันได้ด้วยการเติมประเภทต่อท้ายตัวเลข (เช่น 13u32, 2u8).

fn main() {
    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}
