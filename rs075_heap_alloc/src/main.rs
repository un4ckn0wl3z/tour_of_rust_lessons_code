// การจัดสรรพื้นที่ใน Heap
// Box คือ smart pointer ที่อนุญาตให้เราย้ายข้อมูลจาก stack ไปไว้ใน heap ได้

// Dereferencing ช่วยให้เราสามารถใช้ heap ที่จัดสรรข้อมูลตามหลักสรีรศาสตร์ราวกับว่าเป็นข้อมูลเดิม

struct Pie;

impl Pie {
    fn eat(&self) {
        println!("tastes better on the heap!")
    }
}

fn main() {
    let heap_pie = Box::new(Pie);
    heap_pie.eat();
}

