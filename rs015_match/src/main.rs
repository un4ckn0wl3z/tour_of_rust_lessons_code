// match
// คิดถึง switch statement ของคุณไหมล่ะ? Rust มีคีย์เวิร์ดที่มีประโยชน์อย่างเหลือเชื่อ สำหรับจับคู่เงื่อนไขที่เป็นไปได้ทั้งหมดของค่านั้น และ เรียกใช้โค้ดตามเส้นทางที่จับคู่ได้ว่าเป็นจริง ลองดูจากวิธีที่มันทำงานกับตัวเลขนี้ดูก่อน แล้วเราค่อยมาคุยให้ละเอียดกว่านี้ในบทต่อๆไปภายหลัง ในเรื่องการจับคู่รูปแบบที่ซับซ้อนกว่านี้ ฉันสัญญาว่าคุ้มค่ากับการรอคอยแน่ๆ

// match จะตรวจสอบละเอียดมาก ดังนั้นจึงควรจัดการให้ครอบคลุมทุกกรณี

// match มักจะใช้คู่กับการแยกโครงสร้าง ซึ่งเป็นรูปแบบที่จะพบได้บ่อยๆใน Rust

fn main() {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }
}
