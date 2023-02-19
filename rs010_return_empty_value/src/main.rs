// การไม่คืนอะไรเลย
// ถ้าฟังก์ชันไม่ได้ระบุ type ที่ต้อง return มันจะคืนออกมาเป็น empty tuple หรือที่รู้จักกันอีกชื่อว่า unit

// empty tuple แสดงได้ด้วย ()

// ปกติเราจะไม่ค่อยได้ใช้ () กันหรอกนะ แต่บ่อยครั้งก็ใช้เพื่อให้รู้ว่ามันเกิดอะไรขึ้นจริงๆ

fn make_nothing() -> () {
    return ();
}

// the return type is implied as ()
fn make_nothing2() {
    // this function will return () if nothing is specified to return
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}
