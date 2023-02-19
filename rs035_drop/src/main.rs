// การจัดการรีซอร์สโดยอิงตามขอบเขตข้อมูล
// Rust ใช้จุดสิ้นสุดของขอบเขตเป็นจุดในการ ทำลายโครงสร้าง(deconstruct) และ คืนหน่วยความจำ(deallocate)

// ตอนนี้เราจะเรียกการ ทำลายโครงสร้าง และการคืนหน่วยความจำนี้ว่า drop

// รายละเอียดหน่วยความจำ:

// Rust ไม่มี garbage collection
// เราเรียกสิ่งนี้ว่า Resource Acquisition Is Initialization ( RAII ) ในภาษา C++

struct Foo {
    x: i32,
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);

    println!("{}", foo_b.x);
    // foo_b is dropped here 
    // foo_a is dropped here
}

