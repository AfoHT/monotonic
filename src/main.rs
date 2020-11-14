pub use monotonic::monotonic;

fn main() {
    println!("Hello, monotonic!");

    struct MonotonicImpl {
    }

    #[allow(dead_code)]
    #[monotonic(binds = SysTick)]
    type MyMonotonic = MonotonicImpl;
}
