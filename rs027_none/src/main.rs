// ตัวแทนของความว่างเปล่า
// ในภาษาโปรแกรมมิ่งอื่นจะใช้คีย์เวิร์ด null แทนการไม่มีค่า แต่นั่นก็สร้างความยุ่งยากให้เราอีก ก็เพราะว่า มันเปิดโอกาสให้โปรแกรมเราล้มเหลวถ้าเราดันไปทำอะไรกับค่าพวกนี้เข้า

// ดังนั้น Rust ก็เลยไม่มี null แต่ก็อย่าหาว่าเราละเลยความจำเป็นของการมีอยู่ของ ค่าความว่างเปล่า นะ ลองดูสิ่งที่เรานำเสนอแบบซื่อๆในแบบของเราที่เคยเห็นกันมาแล้ว

// เราเสนอการใช้ None สำหรับค่าที่มีทางเลือกอย่างน้อย 1 อย่างว่าจะออกมาเป็นอะไรกันแน่ ซึ่งเกิดขึ้นได้บ่อยๆใน Rust เพราะเราไม่มี null นั่นเอง Generic types จึงมีบทบาทในการช่วยแก้ไขปัญหาเหล่านี้ให้เรา

enum Item {
    Inventory(String),
    // None represents the absence of an item
    None,
}

struct BagOfHolding {
    item: Item,
}

fn main(){
    let bag_of_holding_1 = BagOfHolding {
        item: Item::Inventory(String::from("Pill"))
    };

    match bag_of_holding_1.item {
        Item::Inventory(item) => println!("Found item: {}", item),
        Item::None => println!("No item found"),
    }

    let bag_of_holding_2 = BagOfHolding {
        item: Item::None
    };

    match bag_of_holding_2.item {
        Item::Inventory(item) => println!("Found item: {}", item),
        Item::None => println!("No item found"),
    }

    
}
