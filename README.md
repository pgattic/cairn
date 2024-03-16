
# Cairn 🪨

Stack-based (esoteric?) programming language

## Description

### Principles:

- The only information allowed on the stack is numbers. Anything else attempting to go onto it must be evaluated to a number.
- Avoid using the `_` (nop) procedure as often as possible.

Example:

```

=fact dup 1 - ?fact:1 *

=main "Hello World!" print _ 5 fact print _ 0 exit

```

## Built-in functions

Note: `t` refers to the value at the top of the stack

- [x] `put` (pops 1, pushes 0): prints the value at the top of the stack.
- [x] `putc` (pops 1, pushes 0): prints the value at the top of the stack as an ASCII value.

- `print` (pops `t+1`, pushes 1): prints to the standard output
- `read` (pops 0, pushes 1): Reads one character from the standard input

### Math

- [x] `+`: add (pops 2, pushes 1)
- [x] `-`: subtract (pops 2, pushes 1) - suntracts value of top of stack from value underneath it
- [x] `*`: multiply (pops 2, pushes 1) - multiplies the top two values
- [x] `div`: divide (pops 2, pushes 1) - integer division
- [ ] `=` (pops 2, pushes 1) - equality

### Control Flow

- [x] `_` (pops 1, pushes 0): no-op 
- [x] `dup` (pops 0, pushes 1): - copies the value at the top of the stack
- [x] `?[val1]:[val2]:...:[valN]` (pops 0, pushes 0): Match statement (described in further detail later)
- [ ] `exit`: (pops 1, pushes 0) Halt program execution, return `t` as the status code
- ``

## Data Types

- int: represented with their integer value
- string: alias for a sequence of ints, followed by its length




