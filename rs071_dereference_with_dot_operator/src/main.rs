// The . Operator
// ตัวดำเนินการ . ถูกใช้เมื่อทำการเข้าถึงฟิลด์หรือเมธอดของตัวอ้างอิง ซึ่งมันค่อนข้างฉลาดอยู่บ้าง

// let f = Foo { value: 42 };
// let ref_ref_ref_f = &&&f;
// println!("{}", ref_ref_ref_f.value);
// ว้าวดูสิ เราไม่ต้องใส่ *** ข้างหน้า ref_ref_ref_f เลยเห็นไหม นี่ก็เพราเจ้าตัวดำเนินการ . มัน dereference ตัวอ้างอิงให้อัตโนมัติ โดยตัวคอมไพเลอร์จะเป็นตัวเปลี่ยนบรรทัดสุดท้ายให้เป็นบรรทัดต่อไปนี้เอง

// println!("{}", (***ref_ref_ref_f).value);

struct Foo {
    value: i32
}

fn main() {
    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", ref_ref_ref_f.value);
}