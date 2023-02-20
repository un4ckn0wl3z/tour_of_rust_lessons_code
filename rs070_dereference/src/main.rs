// The * Operator
// ตัวดำเนินการ * เป็นวิธีที่ชัดเจนที่นำมาใช้ dereference ตัวอ้างอิง

// let a: i32 = 42;
// let ref_ref_ref_a: &&&i32 = &&&a;
// let ref_a: &i32 = **ref_ref_ref_a;
// let b: i32 = *ref_a;
// รายละเอียดหน่วยความจำ:

// เนื่องจาก i32 เป็น type ตั้งต้น ที่อิมพลีเมนต์ trait ชื่อCopy ไบต์ของตัวแปร a บนแสต็กจึงถูกคัดลอกไปยัง ไบต์ของตัวแปร b

fn main() {
    let a: i32 = 42;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a;
    let b: i32 = *ref_a;
    println!("{}", b);

    let x: i32 = 100;
    let y: &i32 = &x;
    let z: i32 = *y;
    println!("{}", z);
}
