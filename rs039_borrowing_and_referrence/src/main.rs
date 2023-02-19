// การยืมความเป็นเจ้าของด้วยวิธีอ้างอิง
// การอ้างอิงถึงช่วยให้เรายืมการเข้าถึงรีซอร์สได้ด้วยการใช้ตัวดำเนินการ &

// การอ้างอิงก็สามารถ drop ได้เหมือนกัน

struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    // f is dropped here
    // foo is dropped here
}

