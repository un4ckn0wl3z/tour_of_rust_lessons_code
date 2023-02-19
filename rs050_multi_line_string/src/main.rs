// Multi-line String Literals
// สตริงใน Rust สามารถมีหลายบรรทัดมาตั้งแต่ต้น

// หากไม่ต้องให้ตัดบรรทัดข้อความ ให้ใช้ \ ปิดท้ายบรรทัด

fn main() {
    let haiku: &'static str = "
        I write, erase, rewrite
        Erase again, and then
        A poppy blooms.
        - Katsushika Hokusai";
    println!("{}", haiku);
    
    
    println!("hello \
    world") // notice that the spacing before w is ignored
}
