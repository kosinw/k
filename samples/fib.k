import std

fn fib(n: i32): i32 {
    // This switch statement is similar to the match statement from Rust
    switch n {
        0 => {
            return 0
        }
        1 => {
            return 1
        }
        _ => {
            return n + fib(n - 1)
        }
    }
}

fn main(): void {
    std.println("The 5th fibonacci number is: " + fib(5))

    let five: i32 = 5; five = 15
}