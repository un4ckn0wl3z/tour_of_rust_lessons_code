// lifetimes ที่ชัดเจน
// ถึงแม้ว่า Rust จะไม่ได้ให้คุณเห็น lifetime ของตัวแปรในโค้ด แต่คอมไพเลอร์เข้าใจเรื่องนี้เป็นอย่างดี และตรวจสอบอยู่เสมอว่าจะไม่มีตัวอ้างอิงใดมี lifetime นานกว่าเจ้าของ

// ฟังก์ชันมีวิธีการกำหนดค่าพารามิเตอร์ให้ชัดเจนด้วยสัญลักษณ์ว่า พารามิเตอร์และค่าที่ส่งคืนจะใช้ lifetime เดียวกัน

// การระบุ lifetime ให้เริ่มด้วย ' (เช่น 'a, 'b, 'c)

struct Foo {
    x: i32,
}

// the parameter foo and return value share the same lifetime
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x is dropped here, allowing us to create a non-mutable reference
    let y = do_something(&foo);
    println!("{}", y);
    // y is dropped here
    // foo is dropped here
}
