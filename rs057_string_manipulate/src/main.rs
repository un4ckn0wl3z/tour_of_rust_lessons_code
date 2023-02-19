// การสร้าง Strings
// concat และ join เป็นสองวิธีง่ายๆที่ดีมากในการสร้างสตริง

fn main() {
    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}",abc);
}

