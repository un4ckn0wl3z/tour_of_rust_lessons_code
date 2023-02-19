// ความเป็นเจ้าของ
// การสร้างอินสแตนซ์สัก type หนึ่ง และ ผูกไว้ กับชื่อตัวแปรจะสร้างรีซอร์สหน่วยความจำ ที่คอมไพเลอร์ Rust จะตรวจสอบตลอด อายุการใช้งาน ตัวแปรที่ถูกผูกไว้ เราจะเรียกว่า เจ้าของ รีซอร์ส

struct Foo {
    x: i32,
}

fn main() {
    // We instantiate structs and bind to variables
    // to create memory resources
    let foo = Foo { x: 42 };
    // foo is the owner
    print!("{}", foo.x);
}
