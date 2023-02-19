// Static Lifetimes
// ตัวแปรแบบ static เป็นรีซอร์สในหน่วยความจำที่ถูกสร้างตั้งแต่ compile-time และจะคงอยู่ ตั้งแต่โปรแกรมเริ่มทำงานจนจบ พวกมันต้องระบุ type ให้ชัดเจน

// static lifetime เป็นรีซอร์สในหน่วยความจำที่จะอยู่ตลอดจนจบโปรแกรม ข้อสังเกตคือ จากนิยามนี้ static lifetime บางตัวอาจจะเกิดตอน runtime ก็ได้

// รีซอร์สของ static lifetime ใช้ 'static เป็นตัวกำหนด

// รีซอร์สของ 'static จะไม่มีวันโดน drop

// ถึงแม้ว่าจะใช้รีซอร์สของ static lifetime มาเก็บการอ้างอิง พวกมันก็ยังต้องเป็น 'static` เหมือนกัน (ถ้าน้อยไปมันจะอยู่ได้ไม่นานพอ)

// รายละเอียดหน่วยความจำ:

// การแก้ไขข้อมูลในตัวแปรแบบ static ที่จริงแล้วเป็นเรื่องที่อันตรายเพราะว่า ปกติตัวแปรนี้เป็น golbal และมันสามารถถูกอ่านได้จากที่ไหนก็ได้ซึ่งอาจจะทำให้เกิดการแย่งข้อมูลกันได้ แล้วเราจะมาพูดกันเรื่องที่ว่า การใช้ตัวแปรแบบ global นี้มีความท้าทายอย่างไรบ้างในภายหลัง
// Rust ยอมให้เราใช้บล็อค unsafe { ... } เพื่อทำบางอย่างที่ตัวคอมไพเลอร์เองไม่อาจรับประกัน ได้ว่าจะเกิดอะไรขึ้นกับหน่วยความจำ และ เรื่อง The R̸͉̟͈͔̄͛̾̇͜U̶͓͖͋̅Ṡ̴͉͇̃̉̀T̵̻̻͔̟͉́͆Ơ̷̥̟̳̓͝N̶̨̼̹̲͛Ö̵̝͉̖̏̾̔M̶̡̠̺̠̐͜Î̷̛͓̣̃̐̏C̸̥̤̭̏͛̎͜O̶̧͚͖͔̊͗̇͠N̸͇̰̏̏̽̃ นี้ เราควรพูดกันแบบจริงจัง

static PI: f64 = 3.1415;

fn main() {
    // static variables can also be scoped to a function
    static mut SECRET: &'static str = "swordfish";

    // string literals have a 'static lifetime
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // You can break some rules, but you must be explicit
    unsafe {
        // we can set SECRET to a string literal because it is also `static
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
}

