// Main ที่ผิดพลาดได้
// main เองก็สามารถคืนค่า Result ได้เช่นกัน

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// Main returns no value, but could return an error!
fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("found {}", v),
        Err(_e) => {
            // handle this error gracefully
            
            // return a new error from main that said what happened!
            return Err(String::from("something went wrong in main!"));
        }
    }

    // Notice we use a unit value inside a Result Ok
    // to represent everything is fine
    Ok(())
}

// https://tourofrust.com/38_th.html