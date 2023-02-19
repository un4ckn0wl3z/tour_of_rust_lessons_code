// ตัวแปร
// การประกาศตัวแปร ใช้คียเวิร์ด let

// ปกติแล้ว Rust จะสามารถอนุมาน type ของตัวแปรที่คุณกำหนดค่าลงไปได้ถึง 99% แต่ถ้าหากมันทำไม่ได้ ก็เพียงแค่เพิ่มการประกาศ type ลงไปด้วยแค่นั้น

// สังเกตดูจะพบว่าเราสามารถ กำหนดค่าให้ตัวแปรชื่อเดิมซ้ำๆ ได้หลายครั้ง สิ่งนี้เราเรียกว่าการทำ shadow ให้ตัวแปร และยังสามารถเปลี่ยน type ของตัวแปรชื่อนั้นได้อีกด้วย

// และชื่อตัวแปรจะใช้ snake_case เสมอนะ

fn main() {
    // rust infers the type of x
    let x = 13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);


}
