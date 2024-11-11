use core::slice;

pub fn make_unsafe_rust() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn try_extern() {
    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }
}

static HELLO_WORLD: &str = "Hello, world!";

pub fn access_static_variable() {
    println!("name is: {HELLO_WORLD}");
}

static mut COUNTER: u32 = 0;

pub fn modify_static_variable(inc: u32) {
    unsafe {
        COUNTER += inc;

        println!("COUNTER: {COUNTER}");
    }
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}
