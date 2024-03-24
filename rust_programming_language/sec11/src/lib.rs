pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        let result = sub(4, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_failed_on_purpose() {
        let result = sub(4, 2);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_panic() {
        panic!("call panic here");
    }

    #[test]
    fn test_error_message() {
        let result = String::from("Hello, here is AA:");
        assert!(
            result.contains("BB"),
            "here is an error and original string is : {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn test_should_panic() {
        panic!("call panic here");
    }


    // if the panic message contains the expected msg, then it will pass test.
    // for example, if the panic msg is "ABCD", then expected "BC" can pass the test.
    // However, the expected "DE" can't pass the test.
    #[test]
    #[should_panic(expected = "call panic here")]
    fn test_should_panic_with_designed_msg() {
        panic!("call panic here ... ");
    }


    #[test]
    fn test_result_type() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("here is error message"))
        }
    }


}
