use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    println!("Async/sync function call benchmark");

    let start = Instant::now();
    async_foo().await?;
    let microseconds = start.elapsed().as_micros();
    println!("microseconds = {}", microseconds);

    let start = Instant::now();
    sync_foo()?;
    let microseconds = start.elapsed().as_micros();
    println!("microseconds = {}", microseconds);

    let start = Instant::now();
    async_foo_sync_bar().await?;
    let microseconds = start.elapsed().as_micros();
    println!("microseconds = {}", microseconds);

    Ok(())
}

async fn async_foo() -> Result<(), Box<dyn std::error::Error>>
{
    println!("Async foo");

    let mut x = 0;

    for _i in 0..10000000 {
        x = async_bar(x).await;
    }

    println!("x = {}", x);

    Ok(())
}

async fn async_foo_sync_bar() -> Result<(), Box<dyn std::error::Error>>
{
    println!("Async foo/sync bar");

    let mut x = 0;

    for _i in 0..10000000 {
        x = sync_bar(x);
    }

    println!("x = {}", x);

    Ok(())
}

async fn async_bar(i: i32) -> i32
{
    i + 1
}

fn sync_foo() -> Result<(), Box<dyn std::error::Error>>
{
    println!("Sync foo");

    let mut x = 0;

    for _i in 0..10000000 {
        x = sync_bar(x);
    }

    println!("x = {}", x);

    Ok(())
}

fn sync_bar(i: i32) -> i32
{
    i + 1
}
