// Generic Types คืออะไร?
// Generic types ช่วยให้เราสามารถ กำหนด struct หรือ enum เฉพาะบางส่วนได้ และยอมให้คอมไพเลอร์ สร้างส่วนที่เหลือให้เสร็จโดยอ้างอิงจากการใช้งานจริงในโค้ด

// โดยปกติแล้ว Rust มีความสามารถในการ อนุมาน type ที่มันเป็นจริงๆได้ โดยดูจากค่าที่เราสร้างอินสแตนซ์นั้นขึ้นมา แต่ถ้าเราอยากที่จะทำให้มันชัดเจนไปเลยก็ให้ใช้ตัวดำเนินการ ::<T> ส่วนมากเราจะเรียกมันว่า turbofish (นี่เพื่อนรักของฉันเลยนะ!)

// A partially defined struct type
struct BagOfHolding<T> {
    item: T,
}

fn main() {
    // Note: by using generic types here, we create compile-time created types. 
    // Turbofish lets us be explicit.
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };
    
    // Rust can infer types for generics too!
    let float_bag = BagOfHolding { item: 3.14 };
    
    // Note: never put a bag of holding in a bag of holding in real life
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );
}

