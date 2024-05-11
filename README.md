# Computer Systems Exploration

## Overview
This project serves as a personal journey to refresh and deepen understanding of computer systems while simultaneously enhancing skills in C and Rust programming languages. It is designed as a learning project where code implementations and theoretical concepts will be explored and documented.

## Objectives
- To revisit and reinforce fundamental concepts of computer systems.
- To practice and improve programming skills in C and Rust.
- To document the learning process and code implementations for future reference.

## Technologies
- **C Programming Language**: Used for low-level system programming to interact directly with hardware and manipulate memory efficiently.
- **Rust Programming Language**: Employed for safe system-level programming, focusing on ownership, concurrency, and safety.

## Project Structure
- **/c**: Contains all C language related code and examples.
- **/rust**: Houses Rust language implementations and examples.
- **/docs**: Includes documentation and notes on computer systems concepts.

## Getting Started
To get started with this project, clone the repository and explore the structured directories.

```bash
git clone https://github.com/yourusername/computer_systems.git
cd computer_systems
```
To run the Metal examples, you need to first compile the Metal shader source files into a binary format that can be used by your application. Here's how you can compile and use the `dotprod.metal` shader:

1. Compile the Metal shader source file to an intermediate AIR file:
   ```bash
   xcrun -sdk macosx metal -c dotprod.metal -o dotprod.air
   ```

2. Convert the AIR file to a Metal Library file:
   ```bash
   xcrun -sdk macosx metallib dotprod.air -o dotprod.metallib
   ```

3. After compiling the shaders, you can check the `main.rs` file in the `metal_examples` directory to see how to use these compiled libraries in your Rust code.

These steps ensure that the Metal shaders are ready to be utilized in your project for GPU-accelerated tasks.

Additional resources for learning and understanding Metal with Rust can be found at the following links:
- [Using Metal and Rust to Make FFT Even Faster](https://blog.lambdaclass.com/using-metal-and-rust-to-make-fft-even-faster/): This blog post provides insights into using Metal with Rust for optimizing FFT computations, which can be beneficial for learning advanced GPU programming techniques.
- [metal-rs GitHub Repository](https://github.com/gfx-rs/metal-rs): This repository contains Rust bindings for Metal, offering various code examples that demonstrate how to use Metal with Rust.
- [Metal Playground Examples](https://github.com/lambdaclass/metal_playground): This example specifically shows how to handle memory in Metal, which is crucial for understanding GPU resource management in system-level programming.
- [Metal Documentation](https://developer.apple.com/documentation/metal): This is the official documentation for Metal.

I went through above resources for understanding of system-level programming with Metal and Rust.

