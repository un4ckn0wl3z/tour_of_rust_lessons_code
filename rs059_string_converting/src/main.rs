// การแปลงค่า Strings
// มีหลาย type ที่สามารถแปลงมาเป็นสตริงได้ด้วยการใช้ to_string

// มี generic ฟังก์ชันตัวนึงคือ parse นำมาใช้แปลงค่าสตริง หรือข้อความสตริง ไปเป็น type อื่นได้ โดยมันจะคืนค่ามาเป็น Result เพราะว่ามันอาจเกิดข้อผิดพลาดได้

fn main() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}

