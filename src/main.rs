pub use monotonic::monotonic;

fn main() {
    println!("Hello, monotonic!");

    #[allow(dead_code)]
    #[monotonic(
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        )]
    struct Monotonic {}

    #[allow(dead_code)]
    #[monotonic(
        monotonic = [ISR, handler]
        monotonic = [ISR, handler]
        )]
    struct Monotonic2 {}
}
