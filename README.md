# Dangling Pointer in Rust

This repository demonstrates a common error in Rust: creating a dangling pointer.  A dangling pointer points to memory that has been freed or deallocated. Dereferencing a dangling pointer leads to undefined behavior, causing crashes or unpredictable results.

The `bug.rs` file shows the erroneous code.  The `bugSolution.rs` file provides a corrected version.

## How to reproduce:

1. Clone the repository.
2. Navigate to the repository's directory.
3. Compile and run `bug.rs` (the buggy code).
4. Observe the potential issues or undefined behavior. You might see a segmentation fault or another program crash depending on your operating system and compiler.
5. Compare the output and behavior with the corrected version in `bugSolution.rs`.