
# Cairn 🪨

Cairn is a stack-oriented programming language, and the interpreter code found here is my first-ever rust project!!!

[Software Demo Video](https://youtu.be/4RxESsPvRfk)

## Description

Cairn is meant to be a general-purpose stack-oriented programming language. All code is meant to be executed onto a single execution stack and evaluated into a single data stack.

My main reason for creating this language was out of curiosity. While reading up on the inner workings of WebAssembly, I read that it uses a stack-based Virtual Machine code. This sounded very interesting to me. I looked at some example WASM assembly code, and realized that theoretically anything can be done in a stack-based language!

So, while this project was mostly an exploratory venture for me, it also resulted in a very lightweight math-focused language that I can use for complex mathematical calculations that I need done quickly. I may use it as a scripting language for other personal projects as an alternative to Python or Bash, if it matures enough and gains general-purpose utility.

### Basics

- All code must be contained in a function.
- The `$main` function is automatically invoked at runtime.
- Whitespace (like newlines, extra spaces) is not significant. There is no concept of code blocks besides a function, so some kind of code nesting using brackets or indentation has no place in this language.

### Principles:

- The only information allowed on the stack is numbers. Anything else attempting to go onto it must be evaluated to a number.
- Avoid using the `_` (drop) function as often as possible.

Example:

```
$fact
  dup -- dup ?++:fact *         # Conditionals and recursion

$say_hi
  "Hello, " print print "!" println

$main
  10 5 + 7 * putln              # Print some math operations

  10000 fact putln              # Use Factorial Function

  "Hello world!" println        # Some stdoutput examples
  "Preston" say_hi
  "What is your name? " println
  readln say_hi
```

## Built-in functions

Note: `t` refers to the value at the top of the stack

- `putln` (pops 1, pushes 0): prints the value at the top of the stack as an integer.
- `println` (pops 1): prints as a string instead of an int
- `readln` (pops 0, pushes 1): Reads a line from the standard input

### Math

- `+`: add (pops 2, pushes 1)
- `-`: subtract (pops 2, pushes 1) - suntracts value of top of stack from value underneath it
- `*`: multiply (pops 2, pushes 1) - multiplies the top two values
- `div`: divide (pops 2, pushes 1) - integer division
- `=` (pops 2, pushes 1) - equality

Other comparison operators have the same syntax that other languages use.

### Control Flow

- `_` (pops 1, pushes 0): drop
- `dup` (pops 0, pushes 1): - copies the value at the top of the stack
- `over` (pops 0, pushes 1): - Copies the second-from-top value to the top of the stack
- `?[val1]:[val2]:...:[valN]` (pops 0, pushes 0): Match statement (TODO: describe in further detail)

## Data Types

- int: represented with their integer value
- string: alias for a sequence of ints, followed by its length

## Development Environment

Cairn was developed using these tools:

- Neovim
- Rust compiler (didn't think I would ever have a favorite compiler, but here I am!)

## Useful Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Let's get Rusty's video about Rust String types](https://youtu.be/CpvzeyzgQdw?si=b6_Z-e7RJNlGbvig)

