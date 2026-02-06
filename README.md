# Learn Rust as a Python Developer

A comprehensive, practical guide for Python developers learning Rust. This guide uses side-by-side comparisons, hands-on projects, and progressive learning to help you master Rust quickly.

## 🎯 What This Guide Offers

- **Concept Mapping**: Direct Python-to-Rust translations of common patterns
- **5 Practical Projects**: Build real tools while learning (CLI apps, web scrapers, APIs, and more)
- **Progressive Learning**: Start simple, build to advanced topics
- **Hands-On Approach**: Learn by doing, not just reading

## 📚 Documentation

The full guide is built with MkDocs and includes:

- **8 Core Chapters**: From basics to advanced concurrency
- **5 Complete Projects**: Each reinforcing specific concepts
- **Quick Reference**: Instant Python-to-Rust pattern lookup
- **Code Examples**: All examples in both Python and Rust

## 🚀 Quick Start

### Prerequisites

- Python experience (2+ years recommended)
- Basic command-line familiarity
- Git installed

### View the Documentation

#### Option 1: View Online (GitHub Pages)

```bash
# Build and view locally
pip install -r requirements.txt
mkdocs serve
```

Then open http://127.0.0.1:8000 in your browser.

#### Option 2: Build Static Site

```bash
pip install -r requirements.txt
mkdocs build
# Site will be in the 'site' directory
```

## 📖 Guide Structure

### Getting Started
1. **Why Rust?** - Understand what Rust offers Python developers
2. **Environment Setup** - Get your development environment ready
3. **First Steps** - Write your first Rust programs

### Core Concepts (Chapters 2-8)
- **Chapter 2**: Basic syntax, variables, and types
- **Chapter 3**: Ownership and borrowing (Rust's superpower)
- **Chapter 4**: Collections and data structures
- **Chapter 5**: Structs, enums, and the type system
- **Chapter 6**: Error handling (Result vs exceptions)
- **Chapter 7**: Async programming and concurrency
- **Chapter 8**: Testing and documentation

### Hands-On Projects
1. **CLI Calculator** - Basic syntax and error handling
2. **File Parser** - Data structures and file I/O
3. **HTTP Client** - Network programming and async
4. **Web Service** - Building REST APIs
5. **Data Pipeline** - Complete concurrent application

## 💡 Learning Philosophy

This guide follows a **concept mapping** approach:

```
Python Concept → Rust Equivalent → Why Different → How to Use
```

For example:
- Python's `try/except` → Rust's `Result<T, E>` → Compile-time vs runtime → Explicit error propagation

## 🎓 Who This Guide Is For

**Perfect for:**
- Python developers wanting systems programming skills
- Engineers needing better performance for specific tasks
- Developers curious about memory safety without GC
- Anyone wanting to add Rust to their toolbox

**You'll get the most out of this if you:**
- Have 2+ years of Python experience
- Understand classes, functions, and basic data structures
- Are comfortable with the command line
- Are willing to embrace a learning curve

## 🛠️ What You'll Build

By the end of this guide, you'll have built:

1. A production-ready CLI calculator
2. A file parsing tool handling various formats
3. An HTTP client with async capabilities
4. A REST API web service
5. A concurrent data processing pipeline

All projects include:
- Complete source code
- Comprehensive tests
- Comparison with Python implementations
- Extension challenges

## 📊 Python vs Rust Quick Comparison

| Aspect | Python | Rust |
|--------|--------|------|
| **Type System** | Dynamic | Static, compile-time |
| **Memory** | Garbage collected | Ownership system |
| **Performance** | ~1x baseline | ~50-200x faster |
| **Concurrency** | GIL-limited | True parallelism |
| **Error Handling** | Exceptions | Result/Option types |
| **Deployment** | Requires runtime | Single binary |

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report issues or suggest improvements
- Submit pull requests with fixes or enhancements
- Share your learning experience

## 📄 License

This project is licensed under the terms specified in the LICENSE file.

## 🔗 Additional Resources

- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings - Interactive Exercises](https://github.com/rust-lang/rustlings)
- [Crates.io - Rust Package Registry](https://crates.io/)

## 🌟 Start Learning

Ready to begin? [Start with the guide](docs/index.md) or jump directly to:

- [Why Rust for Python Developers?](docs/01-foundation/why-rust.md)
- [Environment Setup](docs/01-foundation/environment.md)
- [Quick Reference Guide](docs/reference.md)

---

**Note**: This is a learning resource designed to bridge Python and Rust. Both languages are excellent for different purposes - this guide helps you add Rust to your toolkit effectively.