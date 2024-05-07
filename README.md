
# Cairn ⛰️

([Software Demo Video](https://youtu.be/4RxESsPvRfk))

## Description

Cairn is a general-purpose stack-oriented programming language.

All code is meant to be executed from a single execution stack and ultimately evaluated into a single data stack.

My main reason for creating this language was out of interest in stack-based approaches.

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

- `put`/`putln` (pops 1): outputs the value at the top of the stack as an integer.
- `print`/`println` (pops 1): outputs as a string instead of an int
- `readln` (pops 0, pushes 1): Reads a line from the standard input

### Math

- `++`: increment (pops 1, pushes 1) - increments the value at the top of the stack by 1
- `--`: decrement (pops 1, pushes 1)
- `+`: add (pops 2, pushes 1)
- `-`: subtract (pops 2, pushes 1) - subtracts value of top of stack from value underneath it
- `*`: multiply (pops 2, pushes 1) - multiplies the top two values
- `div`: divide (pops 2, pushes 1) - integer division
- `%`: modulo (pops 2, pushes 1)
- `=`/`!=`/`<`/`>`/`<=`/`>=` (pops 2, pushes 1) - comparison operators (pushes 0 for false and 1 for true)

### Control Flow

- `_` (pops 1, pushes 0): drop
- `dup` (pops 0, pushes 1): copies the value at the top of the stack
- `swp` (pops 2, pushes 2): Switches the location of the top two items in the stack (`a b` -> `b a`)
- `over` (pops 3, pushes 3): Rearranges the top three values of the stack like this: `a b c` -> `b c a` (a moves to the top of the stack)
- `?[val1]:[val2]:...:[valN]` (pops 1, pushes 0): Match statement (TODO: describe in further detail)

## Data Types

- Int (arbitrary size supported)

Strings are syntactic sugar. When the interpreter comes across a string, it converts it into an int according to Unicode standard. `"Hello"` -> `0x48656c6c6f` (310939249775). Ints in Cairn can be arbitrarily large. Part of the principles of this language is that you can really represent any kind of data using a number. The significance of data it is defined by its usage, not its attributes. This is why it has separate functions to print numbers and strings.

There is no collection type. Using the stack to store and manipulate a collection is possible, but the recommended way is to use modular arithmetic to marshal and unmarshal collections of data within a single integer.

Floating-point support is under consideration.

## Development Environment

Cairn was developed using these tools:

- Neovim
- Rust compiler (didn't think I would ever have a favorite compiler, but here I am!)

## Useful Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Let's get Rusty's video about Rust String types](https://youtu.be/CpvzeyzgQdw?si=b6_Z-e7RJNlGbvig)

