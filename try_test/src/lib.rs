pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
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
    fn it_fails() {
        panic!("It will panic!")
    }

    #[test]
    fn it_fails_result() -> Result<(), String> {
        if 2 + 2 == 6 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal six"))
        }
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
