// การส่งพารามิเตอร์ด้วยข้อความ
// โดยปกติ ไม่ว่าจะเป็นข้อความของ String หรือ string ก็สามารถส่งเข้าไปให้ฟังก์ชัน ในรูปแบบ slice ของ string ได้อยู่แล้ว ซึ่งมันช่วยให้ยืดหยุ่นกับทุกสถานการณ์ โดยไม่จำเป็นต้องส่งผ่านความเป็นเจ้าของเข้าไป

fn say_it_loud(msg:&str){
    println!("{}!!!",msg.to_string().to_uppercase());
}

fn main() {
    // say_it_loud can borrow &'static str as a &str
    say_it_loud("hello");
    // say_it_loud can also borrow String as a &str
    say_it_loud(&String::from("goodbye"));
}

