# Immutable by Default: A Paradigm Shift

## The Container Analogy

Imagine you have two types of storage containers:

**Python's Boxes**: You can always change what's inside. The label stays the same, but contents swap freely.

**Rust's Containers**: Come in two flavors:
- **Sealed Boxes** (default): Once filled, contents cannot change
- **Transparent Cases** (`mut`): Contents can be modified, but you must explicitly request this type

## Why This Matters

### Python's Flexibility

```python
temperature = 72
temperature = 75  # Changed my mind
temperature = "hot"  # Changed type too!
```

Every binding is mutable. Every type can change. Maximum flexibility, but harder to reason about.

### Rust's Deliberate Choice

```rust
let temperature = 72;
// temperature = 75;  // Compiler error: cannot modify

let mut adjustable_temp = 72;
adjustable_temp = 75;  // This works - we declared it mutable
```

You must consciously decide: "This value will change over time."

## Real-World Benefit: Tracking State Changes

Consider a weather station that records measurements:

### Python Version

```python
class WeatherStation:
    def __init__(self):
        self.current_temp = 0
        self.readings = []

    def record(self, temp):
        self.current_temp = temp  # Always modifiable
        self.readings.append(temp)

    def analyze(self):
        # Someone could accidentally modify current_temp here
        average = sum(self.readings) / len(self.readings)
        return average
```

Nothing prevents accidental modification of `current_temp` during `analyze()`.

### Rust Version

```rust
struct WeatherStation {
    current_temp: i32,  // Immutable field
    readings: Vec<i32>,
}

impl WeatherStation {
    fn new() -> Self {
        WeatherStation {
            current_temp: 0,
            readings: Vec::new(),
        }
    }

    fn record(&mut self, temp: i32) {
        // Can only modify because &mut self
        self.current_temp = temp;
        self.readings.push(temp);
    }

    fn analyze(&self) -> f64 {
        // &self means read-only - cannot modify anything
        let total: i32 = self.readings.iter().sum();
        total as f64 / self.readings.len() as f64
    }
}
```

The `&self` vs `&mut self` distinction makes modifications intentional and visible.

## Shadowing: Rust's Unique Feature

Shadowing lets you reuse variable names with different values or types:

```rust
let measurement = "25.5";  // String slice
let measurement = measurement.parse::<f64>().unwrap();  // Now f64
let measurement = measurement as i32;  // Now i32
```

Each `let` creates a **new variable** that shadows the previous one. The old value is gone.

Think of it like putting a new label on top of an old one - you can only see the top label.

### When Shadowing Helps

```rust
fn process_sensor_input(raw_data: &str) -> Result<i32, String> {
    // Step 1: Raw string
    let value = raw_data.trim();

    // Step 2: Parse to float
    let value = value.parse::<f64>()
        .map_err(|_| "Invalid number format")?;

    // Step 3: Round to integer
    let value = value.round() as i32;

    // Step 4: Validate range
    let value = if value >= 0 && value <= 100 {
        Ok(value)
    } else {
        Err("Value out of range")
    }?;

    Ok(value)
}
```

Same name, different stages of processing. Each transformation is clear.

## Practical Example: Game Score Tracker

Let's build something that tracks game scores across rounds.

### Python Implementation

```python
class ScoreTracker:
    def __init__(self, player_name):
        self.player_name = player_name
        self.scores = []
        self.bonus_multiplier = 1.0

    def add_score(self, points):
        self.scores.append(points * self.bonus_multiplier)

    def set_bonus(self, multiplier):
        self.bonus_multiplier = multiplier

    def total(self):
        return sum(self.scores)

    def reset(self):
        self.scores = []
        self.bonus_multiplier = 1.0

# Usage
tracker = ScoreTracker("Alice")
tracker.add_score(100)
tracker.set_bonus(1.5)
tracker.add_score(100)  # Gets multiplied by 1.5
print(tracker.total())  # 250
```

Everything is mutable all the time.

### Rust Implementation

```rust
struct ScoreTracker {
    player_name: String,
    scores: Vec<f64>,
    bonus_multiplier: f64,
}

impl ScoreTracker {
    fn new(player_name: String) -> Self {
        ScoreTracker {
            player_name,
            scores: Vec::new(),
            bonus_multiplier: 1.0,
        }
    }

    fn add_score(&mut self, points: f64) {
        // &mut required to modify
        self.scores.push(points * self.bonus_multiplier);
    }

    fn set_bonus(&mut self, multiplier: f64) {
        // &mut required to modify
        self.bonus_multiplier = multiplier;
    }

    fn total(&self) -> f64 {
        // &self means read-only - safe to call anytime
        self.scores.iter().sum()
    }

    fn player(&self) -> &str {
        // &self - just reading
        &self.player_name
    }

    fn reset(&mut self) {
        // &mut required to modify
        self.scores.clear();
        self.bonus_multiplier = 1.0;
    }
}

// Usage
let mut tracker = ScoreTracker::new(String::from("Alice"));
tracker.add_score(100.0);
tracker.set_bonus(1.5);
tracker.add_score(100.0);
println!("Total: {}", tracker.total());  // 250
```

The `&mut` annotations make it clear which functions can modify state.

## The Mental Model: Mailboxes

Think of variables as mailboxes:

**Immutable (default)**:
- Mailbox is locked
- Contents visible but untouchable
- Want to change? Create a new mailbox

**Mutable (`mut`)**:
- Mailbox has a key
- You can swap contents
- But only one person can have the key at a time (enforced by borrow checker)

## Common Patterns

### Pattern 1: Transform and Reassign

Python:
```python
data = load_from_file()
data = clean_data(data)
data = transform(data)
result = analyze(data)
```

Rust equivalent with shadowing:
```rust
let data = load_from_file();
let data = clean_data(data);
let data = transform(data);
let result = analyze(data);
```

Or with mutation:
```rust
let mut data = load_from_file();
clean_data_in_place(&mut data);
transform_in_place(&mut data);
let result = analyze(&data);
```

### Pattern 2: Configuration Builder

```rust
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
}

impl ServerConfig {
    fn new() -> Self {
        ServerConfig {
            host: String::from("localhost"),
            port: 8080,
            max_connections: 100,
        }
    }

    // Each method takes ownership and returns modified config
    fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self  // Return ownership
    }

    fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }
}

// Chaining pattern
let config = ServerConfig::new()
    .with_host(String::from("0.0.0.0"))
    .with_port(3000)
    .with_max_connections(500);
```

This builder pattern is impossible without `mut` in the method signature.

### Pattern 3: Accumulator

```rust
fn calculate_metrics(measurements: &[f64]) -> (f64, f64, f64) {
    let mut sum = 0.0;
    let mut min_val = f64::MAX;
    let mut max_val = f64::MIN;

    for &measure in measurements {
        sum += measure;
        if measure < min_val {
            min_val = measure;
        }
        if measure > max_val {
            max_val = measure;
        }
    }

    (min_val, max_val, sum / measurements.len() as f64)
}
```

Accumulators need `mut` because they change during iteration.

## Debugging Tip: When to Use Mut

Ask yourself:

1. **Does this value change over time?** -> Use `mut`
2. **Is it computed once and read many times?** -> Leave immutable
3. **Am I transforming data in a pipeline?** -> Consider shadowing
4. **Am I accumulating results in a loop?** -> Needs `mut`

## Exercise: Sensor Data Buffer

Create a circular buffer that stores the last N sensor readings:

```rust
struct SensorBuffer {
    readings: Vec<f64>,
    capacity: usize,
    current_index: usize,
}

impl SensorBuffer {
    fn new(capacity: usize) -> Self {
        // Your implementation
    }

    fn push(&mut self, reading: f64) {
        // Add reading, overwrite oldest if full
    }

    fn average(&self) -> f64 {
        // Calculate average of all readings
    }

    fn latest(&self) -> Option<f64> {
        // Return most recent reading
    }
}
```

Which methods need `&mut self` vs `&self`?

<details>
<summary>Solution Hint</summary>

- `new`: No self at all (creates new instance)
- `push`: Needs `&mut self` (modifies buffer)
- `average`: Needs `&self` (only reads)
- `latest`: Needs `&self` (only reads)

</details>

## Key Takeaways

1. **Immutability is the default** - opt into mutation with `mut`
2. **Shadowing allows transformation** - new variable, same name
3. **Method signatures signal intent** - `&self` vs `&mut self`
4. **Compiler enforces discipline** - prevents accidental modifications
5. **Better reasoning** - know exactly where state changes

## Next Topic

Now that you understand immutability, explore [Ownership & Borrowing](03-ownership.md) to see how Rust's type system builds on this foundation.

---

**Remember**: Rust's default immutability isn't about restriction - it's about clarity. When everything can change, nothing is predictable. When only declared-mutable things change, your code becomes easier to reason about.
