// การอ้างอิงถึงตัวอ้างอิงตัวอื่น
// การอ้างอิงสามารถใช้อ้างถึงการอ้างอิงอื่นได้

struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x is dropped here allow us to create a non-mutable reference
    let y = do_something(&foo);
    println!("{}", y);
    // y is dropped here
    // foo is dropped here
}

