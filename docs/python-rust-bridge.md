# The Python-Rust Bridge

## Two Languages, One Toolbox

Think of Python and Rust as complementary tools rather than competitors. Python excels at rapid iteration and expressiveness. Rust delivers performance and compile-time correctness.

## Where Each Language Shines

### Python's Sweet Spot

```python
# Data science pipeline
import pandas as pd
import matplotlib.pyplot as plt

weather_data = pd.read_csv('temperatures.csv')
monthly_avg = weather_data.groupby('month')['temp'].mean()
monthly_avg.plot(kind='bar')
plt.show()
```

Python wins when:
- Prototyping ideas quickly
- Leveraging rich data science libraries
- Writing glue code between systems
- Interactive development and experimentation

### Rust's Territory

```rust
// High-performance data processor
use rayon::prelude::*;

fn process_sensor_batch(readings: &[SensorData]) -> Vec<ProcessedResult> {
    readings.par_iter()
        .map(|reading| intensive_calculation(reading))
        .collect()
}
```

Rust excels at:
- CPU-intensive computations
- Systems requiring strong reliability guarantees
- Applications needing predictable latency
- Tools distributed as standalone binaries

## The Performance Story

Let's compare a realistic scenario: processing a million temperature readings.

### Python Implementation

```python
def calculate_heat_index(temp_celsius, humidity_percent):
    temp_f = (temp_celsius * 9/5) + 32
    hi = -42.379 + 2.04901523*temp_f + 10.14333127*humidity_percent
    # ... more complex formula
    return hi

readings = [(temp, hum) for temp, hum in load_data()]
heat_indices = [calculate_heat_index(t, h) for t, h in readings]
```

Performance: ~2.5 seconds for 1M calculations

### Rust Implementation

```rust
fn calculate_heat_index(temp_celsius: f64, humidity_percent: f64) -> f64 {
    let temp_f = (temp_celsius * 9.0/5.0) + 32.0;
    let hi = -42.379 + 2.04901523*temp_f + 10.14333127*humidity_percent;
    // ... more complex formula
    hi
}

let heat_indices: Vec<f64> = readings.par_iter()
    .map(|(t, h)| calculate_heat_index(*t, *h))
    .collect();
```

Performance: ~0.015 seconds for 1M calculations

**That's 150x faster!** And this is without extensive optimization.

## Bridging the Gap: PyO3

You can call Rust from Python, combining both strengths:

```rust
// Rust library exposed to Python
use pyo3::prelude::*;

#[pyfunction]
fn fast_heat_calculator(temps: Vec<f64>, humidities: Vec<f64>) -> Vec<f64> {
    temps.iter()
        .zip(humidities.iter())
        .map(|(t, h)| calculate_heat_index(*t, *h))
        .collect()
}

#[pymodule]
fn weather_calc(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(fast_heat_calculator, module)?)?;
    Ok(())
}
```

Then use it in Python:

```python
import weather_calc

temps = [25.0, 28.0, 32.0, ...]
humidities = [60.0, 65.0, 70.0, ...]
results = weather_calc.fast_heat_calculator(temps, humidities)
```

Get Python's ease with Rust's speed!

## Memory Management Philosophy

### Python's Approach: Automatic Management

```python
grocery_list = ['apples', 'bread', 'milk']
shared_list = grocery_list  # Both variables point to same list
grocery_list.append('eggs')
print(shared_list)  # ['apples', 'bread', 'milk', 'eggs']
# Garbage collector handles cleanup
```

Benefits:
- Simple mental model
- No manual memory management
- Easy sharing of data

Trade-offs:
- Runtime overhead from GC
- Unpredictable pause times
- Hidden memory usage patterns

### Rust's Approach: Compile-Time Tracking

```rust
let grocery_list = vec!["apples", "bread", "milk"];
let shared_list = grocery_list;  // Ownership transferred
// grocery_list is no longer valid!

// To share, we borrow:
let grocery_list = vec!["apples", "bread", "milk"];
let shared_view = &grocery_list;  // Borrow, don't take
println!("{:?}", grocery_list);  // Still works
println!("{:?}", shared_view);   // Also works
```

Benefits:
- Zero runtime overhead
- Predictable performance
- Impossible to leak memory
- Thread safety guaranteed

Trade-offs:
- Steeper learning curve
- More planning required
- Compiler enforces discipline

## Deployment Comparison

### Python Application

```bash
# User must have Python installed
# Dependencies need to be installed
pip install -r requirements.txt
python app.py

# OR use PyInstaller/cx_Freeze
# Results in large bundle (~50-100MB)
```

### Rust Application

```bash
# Build once
cargo build --release

# Single binary, no dependencies
./target/release/app

# Typical size: 2-5MB
# No runtime needed
```

Rust binaries are:
- Standalone executables
- Fast startup (no interpreter)
- Small file size
- Cross-compilable for different platforms

## Concurrency Models

### Python: Limited by GIL

```python
import threading

def crunch_numbers(data_chunk):
    # CPU-bound work
    return sum(x**2 for x in data_chunk)

# Multiple threads, but only one executes Python at a time
threads = [threading.Thread(target=crunch_numbers, args=(chunk,))
           for chunk in data_chunks]
```

The Global Interpreter Lock (GIL) prevents true parallelism for CPU-bound tasks.

### Rust: True Parallelism

```rust
use rayon::prelude::*;

fn crunch_numbers(data_chunk: &[i32]) -> i32 {
    data_chunk.iter().map(|x| x * x).sum()
}

// Automatically parallelized across all CPU cores
let results: Vec<i32> = data_chunks.par_iter()
    .map(|chunk| crunch_numbers(chunk))
    .collect();
```

All cores working simultaneously, with guaranteed thread safety.

## Type System Differences

### Python: Optional Type Hints

```python
def process_recipe(name: str, servings: int) -> dict:
    """Type hints are optional and not enforced at runtime"""
    return {
        'name': name,
        'servings': servings,
        'calories_per_serving': 350
    }

# This will run, despite wrong types!
result = process_recipe(['invalid'], 'wrong')
```

### Rust: Enforced Types

```rust
struct Recipe {
    name: String,
    servings: u32,
    calories_per_serving: u32,
}

fn process_recipe(name: String, servings: u32) -> Recipe {
    Recipe {
        name,
        servings,
        calories_per_serving: 350,
    }
}

// Won't compile with wrong types
// let result = process_recipe(vec!["invalid"], "wrong");
```

The compiler guarantees type correctness.

## When to Choose What

### Use Python When:
- Rapid prototyping is the priority
- Rich library ecosystem is needed (data science, ML)
- Performance isn't critical
- Team unfamiliar with systems programming
- Quick iteration matters more than optimization

### Use Rust When:
- Performance is crucial
- Memory safety is paramount
- Concurrent processing is needed
- Distributing compiled binaries
- Long-running services requiring reliability

### Use Both When:
- Python for high-level logic
- Rust for performance-critical components
- Leverage strengths of each language

## Real-World Patterns

### Pattern 1: Python Orchestration + Rust Workers

```
Python Script (Orchestrator)
    |-> Loads configuration
    |-> Calls Rust library for heavy processing
    |-> Handles results and visualization
    +-> Generates reports
```

### Pattern 2: Rust Service + Python Clients

```
Rust Web Service (High-performance API)
    ^
    +-- Python clients make requests
       (Web scrapers, data pipelines, etc.)
```

### Pattern 3: Gradual Migration

```
Python Codebase
    |-> Identify bottlenecks
    |-> Rewrite critical paths in Rust
    |-> Expose via PyO3
    +-> Keep Python for everything else
```

## Next Steps

Understanding this bridge is crucial. Now let's [get started with Rust](01-getting-started.md) and start writing code!

---

**Key Takeaway**: Don't think "Python OR Rust" - think "Python AND Rust" for maximum effectiveness.
