pub fn try_type_alias() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

pub fn try_type_alias1() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}

    fn returns_long_type() -> Thunk {
        let t: Thunk = Box::new(|| println!("mom"));

        return t;
    }
}
