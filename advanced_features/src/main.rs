mod unsafe_rust;

fn main() {
    // unsafe_rust::unsafe_rust::make_unsafe_rust();
    unsafe_rust::unsafe_rust::split_at_mut(&mut [10, 11, 12], 3);
}
