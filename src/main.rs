pub use monotonic::monotonic;

fn main() {
    println!("Hello, monotic!");

    #[allow(dead_code)]
    #[monotonic(
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        interruptname=ISR, handlerimpl=handler
        )]
    struct Monotonic {}
}
