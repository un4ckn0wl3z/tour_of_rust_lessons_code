// String Literals From Files
// หากคุณมีข้อความขนาดใหญ่มากให้ลองใช้มาโคร include_str! เพื่อนำข้อความจากไฟล์ในเครื่อง เข้ามาในโปรแกรมของคุณ:

// let hello_html = include_str!("hello.html");

fn main() {
    let hello_html = include_str!("hello.html");
    println!("{}", hello_html);

}
