// การ drop ตามลำดับชั้น
// เมื่อ struct ถูก drop ลง ตัว struct เองจะถูก drop ก่อน จากนั้นลูกๆของมันจึงจะ drop ตัวเอง และเป็นแบบนี้ไปเรื่อยๆ

// รายละเอียดหน่วยความจำ:

// การที่ Rust ใช้วิธีคืนหน่วยความจำโดยอัตโนมัติ จะช่วยให้มั่นใจได้ว่าจะเกิดการรั่วน้อยลง
// การ drop ทำได้เพียงครั้งเดียว

struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo is dropped first
    // then foo.bar is dropped
}

