// Raw String Literals
// Raw strings ทำให้เราเขียนลำดับอักขระแบบตรงตัวได้โดยให้ขึ้นต้นด้วย r#" และปิดท้ายด้วย "# ทำแบบนี้ช่วยให้เราสามารถแทรกอักขระอาจทำให้สับสนว่า มันอยู่ในฐานะสตริง หรือควรเป็นข้อความ (อย่างเช่นเครื่องหมายคำพูด และ แบ็กสแลช)

fn main() {
    let a: &'static str = r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
        "#;
    println!("{}", a);
}

