// การจัดการ Error แบบสวยงาม
// อย่างที่รู้ว่า Result เป็นของกลางๆ Rust จึงมีตัวดำเนินการ ? เอาไว้ใช้คู่กับมัน ทำให้สองคำสั่งนี้ทำงานเหมือนกันเลย:

// do_something_that_might_fail()?
// match do_something_that_might_fail() {
//     Ok(v) => v,
//     Err(e) => return Err(e),
// }


fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // Look at how much code we saved!
    let v = do_something_that_might_fail(40)?;
    println!("found {}", v);
    Ok(())
}

