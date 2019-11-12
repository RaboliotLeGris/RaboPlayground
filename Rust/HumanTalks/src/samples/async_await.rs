use futures;

pub fn run() {
    let task = foo();

    futures::executor::block_on(async {
        let v = task.await;
        println!("Result: {}", v);
    });
}

async fn long_running_operation(a: u8, b: u8) -> u8 {
    println!("long_running_operation");
    a + b
}

fn another_operation(c: u8, d: u8) -> u8 {
    println!("another_operation");
    c * d
}

async fn foo() -> u8 {
    println!("foo");
    let sum = long_running_operation(1, 2);
    another_operation(3, 4);
    sum.await
}
