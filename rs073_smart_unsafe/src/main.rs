// Smart Unsafe Code
// เรามักจะใช้ Smart pointers คู่กับ unsafe ในการเขียนโค้ดอยู่บ่อยครั้ง แต่ก็อย่างที่ได้กล่าวมาแล้วก่อนหน้านี้ว่า ของพวกนี้เป็นเครื่องมือพื้นฐานที่ Rust ใช้ โต้ตอบกับหน่วยความจำในระดับล่างที่สุด

// แล้วโค้ดที่ไม่ปลอดภัย (unsafe code) นี่คืออะไรน่ะเหรอ ที่จริงมันก็คือโค้ด Rust ปกติธรรมดาแต่มีข้อยกเว้นบางอย่างที่คอมไพเลอร์ของ Rust ไม่อาจรับประกันได้

// ความสามารถหลักของ unsafe code ก็คือ dereferencing a raw pointer (การจัดการ raw pointer) ซึ่งก็คือการใช้ raw pointer ขี้ไปที่ตำแหน่งในหน่วยความจำและประกาศว่า "มีโครงสร้างข้อมูลอยู่ตรงนี้!" และเปลี่ยนมันมาอยู่ในรูปที่คุณเอาไปใช้ได้ (เช่นเปลี่ยน *const u8 ไปเป็น u8) Rust ไม่มีความสามารถที่จะตามไปดูว่าแต่ละไบต์ที่ถูกเขียนลงไปในหน่วยความจำมีความหมายว่าอย่างไรบ้าง เพราะว่า Rust ไม่สามารถรับประกันสิ่งที่อยู่ในเลขที่ถูกใช้เป็น raw pointer เราจึงเอา dereference นั้นไปใส่ไว้ในกล่อง unsafe { ... }

// การใช้ Smart pointers เพื่อ dereference ไปที่ raw pointers แบบนี้มันชัดเจนดีว่ากำลังต้องการทำอะไร

fn main() {
    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is totally safe
    let pointer_a = &a as *const u8 as usize;
    println!("Data memory location: {}", pointer_a);
    // Turning our number into a raw pointer to a f32 is
    // also safe to do.
    let pointer_b = pointer_a as *const f32;
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);
}

