use std::error::Error;

pub fn result() {
    match step1() {
        Ok(t) => println!("Step1 worked"),
        _ => println!("Step1 failed"),
    }
//    let result_panic = panic().unwrap(); // bad example causes panic
}

fn step1() -> Result<(), Box<dyn Error>> {
    let res_step2 = step2()?;
    println!("Step2 result: {}", res_step2);
    Ok(())
}

fn step2() -> Result<String, Box<dyn Error>> {
    Ok(String::from("Ok"))
}

fn panic() -> Result<(), Box<dyn Error>> {
    Err("Nasty error".into())
}

#[cfg(test)]
mod result_tests {
    use super::step1;
    #[test]
    fn step1_work() {
        let result_step1 = step1();
        assert!(result_step1.is_ok());
    }
}