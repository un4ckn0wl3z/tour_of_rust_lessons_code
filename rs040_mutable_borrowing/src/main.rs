// การยืมความเป็นเจ้าของ กับของที่เปลี่ยนแปลงค่าได้ ด้วยวิธีอ้างอิง
// นอกจากนี้เรายังสามารถยืมการเข้าถึงรีซอร์สที่เปลี่ยนค่าได้ด้วยตัวดำเนินการ &mut

// โดยที่ตัวเจ้าของเองจะไม่สามารถย้ายหรือแก้ไขอะไรได้ในขณะที่ถูกยืม

// รายละเอียดหน่วยความจำ:

// Rust ป้องกันการเข้าถึงจากทั้งสองทางให้กับ คนที่จะเปลี่ยนค่า กับตัวเจ้าของเอง เพราะว่า ไม่เช่นนั้นอาจทำให้เกิดปัญหาแย่งข้อมูลกัน(data race)

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // FAILURE: do_something(foo) would fail because
    // foo cannot be moved while mutably borrowed

    // FAILURE: foo.x = 13; would fail here because
    // foo is not modifiable while mutably borrowed

    f.x = 13;
    // f is dropped here because it's no longer used after this point
    
    println!("{}", foo.x);
    
    // this works now because all mutable references were dropped
    foo.x = 7;
    
    // move foo's ownership to a function
    do_something(foo);
}

