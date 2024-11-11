fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

pub fn try_function_pointers() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}

pub fn try_return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
