# Concurrency

Python's concurrency story is complicated — the GIL limits true parallelism, `asyncio` handles I/O-bound work, and `multiprocessing` is the escape hatch for CPU-bound work. Rust gives you fearless concurrency: threads, async, and channels — all checked at compile time.

## Python's Concurrency Landscape

| Approach | Best for | GIL? |
|----------|----------|------|
| `threading` | I/O-bound (sort of) | Limited by GIL |
| `asyncio` | I/O-bound (many connections) | Single-threaded |
| `multiprocessing` | CPU-bound | Separate processes |

## Rust's Concurrency Landscape

| Approach | Best for | Safety |
|----------|----------|--------|
| `std::thread` | CPU-bound parallelism | Compile-time checked |
| `async/await` (tokio) | I/O-bound (many connections) | Compile-time checked |
| Channels | Thread communication | Type-safe |
| `Arc`/`Mutex` | Shared state | Compile-time checked |

The key difference: **Rust has no GIL**. Threads run truly in parallel. And the ownership system prevents data races at compile time.

## Threads

=== "Rust"

    ```rust
    use std::thread;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 0..5 {
                println!("Thread: {i}");
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        for i in 0..3 {
            println!("Main: {i}");
            thread::sleep(std::time::Duration::from_millis(150));
        }

        handle.join().unwrap();  // wait for thread to finish
    }
    ```

=== "Python"

    ```python
    import threading
    import time

    def worker():
        for i in range(5):
            print(f"Thread: {i}")
            time.sleep(0.1)

    t = threading.Thread(target=worker)
    t.start()

    for i in range(3):
        print(f"Main: {i}")
        time.sleep(0.15)

    t.join()
    ```

### Moving Data Into Threads

The ownership system shines here. You must explicitly move data into threads:

```rust
use std::thread;

let name = String::from("Alice");

// `move` transfers ownership to the thread
let handle = thread::spawn(move || {
    println!("Hello from thread, {name}!");
});

// println!("{name}");  // ERROR: name was moved

handle.join().unwrap();
```

In Python, threads share memory freely — which is the source of most concurrency bugs. Rust makes shared mutable state explicit and safe.

## Shared State: Arc and Mutex

When multiple threads need access to the same data, Rust requires explicit synchronization:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc: Atomic Reference Counted (thread-safe Rc)
    // Mutex: Mutual Exclusion lock
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);  // clone the Arc, not the data
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());  // 10
}
```

| Rust | Python | Purpose |
|------|--------|---------|
| `Arc<T>` | Shared reference (automatic in Python) | Thread-safe shared ownership |
| `Mutex<T>` | `threading.Lock()` | Mutual exclusion |
| `RwLock<T>` | `threading.RLock()` (roughly) | Read-write lock |
| `Arc<Mutex<T>>` | Shared mutable object | Thread-safe shared mutable state |

!!! tip "The compiler prevents data races"
    Try to share a `Vec` between threads without `Arc<Mutex<>>` — the compiler will refuse. This is ownership in action: compile-time prevention of data races.

## Channels: Message Passing

Channels let threads communicate by sending messages, similar to Go's channels or Python's `queue.Queue`:

=== "Rust"

    ```rust
    use std::sync::mpsc;  // multiple producer, single consumer
    use std::thread;

    fn main() {
        let (tx, rx) = mpsc::channel();

        // Spawn a producer thread
        thread::spawn(move || {
            let messages = vec!["hello", "from", "thread"];
            for msg in messages {
                tx.send(msg).unwrap();
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        // Receive in main thread
        for received in rx {
            println!("Got: {received}");
        }
    }
    ```

=== "Python"

    ```python
    import queue
    import threading
    import time

    q = queue.Queue()

    def producer():
        for msg in ["hello", "from", "thread"]:
            q.put(msg)
            time.sleep(0.1)
        q.put(None)  # signal completion

    threading.Thread(target=producer).start()

    while True:
        msg = q.get()
        if msg is None:
            break
        print(f"Got: {msg}")
    ```

### Multiple Producers

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

for id in 0..3 {
    let tx = tx.clone();  // clone the sender for each thread
    thread::spawn(move || {
        tx.send(format!("Message from thread {id}")).unwrap();
    });
}

drop(tx);  // drop original sender so rx knows when all senders are done

for msg in rx {
    println!("{msg}");
}
```

## Async/Await: Rust's asyncio

Rust has `async/await` syntax similar to Python's. The biggest difference: Rust doesn't have a built-in async runtime. You choose one — **tokio** is the most popular.

### Setup

```bash
cargo add tokio --features full
```

### Basic Async

=== "Rust (with tokio)"

    ```rust
    use tokio::time::{sleep, Duration};

    async fn fetch_data(id: u32) -> String {
        sleep(Duration::from_secs(1)).await;
        format!("Data for {id}")
    }

    #[tokio::main]
    async fn main() {
        let result = fetch_data(1).await;
        println!("{result}");
    }
    ```

=== "Python"

    ```python
    import asyncio

    async def fetch_data(id: int) -> str:
        await asyncio.sleep(1)
        return f"Data for {id}"

    async def main():
        result = await fetch_data(1)
        print(result)

    asyncio.run(main())
    ```

### Concurrent Async Tasks

=== "Rust"

    ```rust
    use tokio::time::{sleep, Duration};

    async fn fetch(url: &str) -> String {
        sleep(Duration::from_millis(500)).await;
        format!("Response from {url}")
    }

    #[tokio::main]
    async fn main() {
        // Run concurrently (like asyncio.gather)
        let (r1, r2, r3) = tokio::join!(
            fetch("https://api.example.com/1"),
            fetch("https://api.example.com/2"),
            fetch("https://api.example.com/3"),
        );

        println!("{r1}");
        println!("{r2}");
        println!("{r3}");
    }
    ```

=== "Python"

    ```python
    import asyncio

    async def fetch(url: str) -> str:
        await asyncio.sleep(0.5)
        return f"Response from {url}"

    async def main():
        r1, r2, r3 = await asyncio.gather(
            fetch("https://api.example.com/1"),
            fetch("https://api.example.com/2"),
            fetch("https://api.example.com/3"),
        )
        print(r1)
        print(r2)
        print(r3)

    asyncio.run(main())
    ```

### Spawning Tasks

```rust
use tokio;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        // tokio::spawn is like asyncio.create_task
        let handle = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100 * i)).await;
            format!("Task {i} done")
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        println!("{result}");
    }
}
```

## Rayon: Effortless Parallelism

The `rayon` crate makes parallel iteration trivial — like having a parallel version of Python's list comprehensions:

```bash
cargo add rayon
```

```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i64> = (0..1_000_000).collect();

    // Sequential
    let sum: i64 = numbers.iter().sum();

    // Parallel — just change .iter() to .par_iter()!
    let sum: i64 = numbers.par_iter().sum();

    // Parallel map + filter
    let results: Vec<i64> = numbers.par_iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
}
```

This is what Python's `multiprocessing.Pool.map()` tries to do, but with zero overhead and type safety.

## When to Use What

| Situation | Python | Rust |
|-----------|--------|------|
| CPU-bound parallel work | `multiprocessing` | `std::thread` or `rayon` |
| Many network requests | `asyncio` + `aiohttp` | `tokio` + `reqwest` |
| Background task | `threading.Thread` | `std::thread::spawn` |
| Producer/consumer | `queue.Queue` | `mpsc::channel` |
| Shared counter | `threading.Lock` | `Arc<Mutex<T>>` |
| Parallel iteration | `multiprocessing.Pool` | `rayon::par_iter()` |

## Exercises

1. Write a program that spawns 5 threads, each computing the sum of a slice of a large vector. Combine the results in the main thread
2. Use channels to implement a producer-consumer pattern: one thread generates numbers, another filters evens, a third prints them
3. Write an async program using tokio that "fetches" 5 URLs concurrently (simulate with `tokio::time::sleep`)
4. Use rayon to parallelize a word-count operation across multiple files

**Next up**: [Testing](11-testing.md) — from pytest to cargo test.
