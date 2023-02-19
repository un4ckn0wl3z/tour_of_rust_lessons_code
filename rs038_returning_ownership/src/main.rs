// การกลับมาเป็นเจ้าของ
// แน่นอนว่า ความเป็นเจ้าของก็สามารถได้คืนจากฟังก์ชันได้

struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // ownership is moved out
}

fn main() {
    let foo = do_something();
    // foo becomes the owner
    println!("{}", foo.x);
    // foo is dropped because of end of function scope
}
