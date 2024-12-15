# Rusticle Language Syntax

Rusticle is a custom language interpreter with a unique syntax. This document provides an overview of the syntax and usage of the Rusticle language.

## Example Program

Here is an example program that demonstrates the syntax of Rusticle:

```rust
import "add" from "addition";

// declare variables
manle a = 6;
manle b = 7;
manle c = a + b;

// conditional statements
agar (c < 0) {
    likh c;
} nhito {
    likh b;
}

// function declaration
functio check(a) {
    likh a;
}

// function calls
check(1);

// loops
for (manle i = 0; i < 3; i = i + 1) {
  likh i;
}

add(11, 11);
```

## Keywords

The following keywords are reserved in Rusticle:

- `and`
- `class`
- `nhito` (else)
- `false`
- `for`
- `functio` (function)
- `agar` (if)
- `nil`
- `or`
- `likh` (print)
- `dede` (return)
- `super`
- `this`
- `true`
- `manle` (var)
- `jabTak` (while)
- `import`
- `from`

## Variables

Variables are declared using the `manle` keyword:

```rust
manle a = 10;
manle b = 20;
manle c = a + b;
```

## Functions

Functions are declared using the `functio` keyword:

```rust
functio add(a, b) {
    likh a + b;
}
```

Functions can be called with arguments:

```rust
add(5, 10);
```

## Conditional Statements

Conditional statements use the `agar` and `nhito` keywords:

```rust
agar (a > b) {
    likh "a is greater than b";
} nhito {
    likh "a is not greater than b";
}
```

## Loops

### While Loop

The `jabTak` keyword is used for while loops:

```rust
jabTak (a < 10) {
    likh a;
    a = a + 1;
}
```

### For Loop

For loops are structured as follows:

```rust
for (manle i = 0; i < 5; i = i + 1) {
    likh i;
}
```

## Logical Operators

Rusticle supports the following logical operators:

- `and`
- `or`

Example:

```rust
agar (a > 0 and b > 0) {
    likh "Both a and b are positive";
}
```

## Comparison Operators

Rusticle supports the following comparison operators:

- `>`
- `>=`
- `<`
- `<=`
- `==`
- `!=`

Example:

```rust
agar (a == b) {
    likh "a is equal to b";
}
```

## Arithmetic Operators

Rusticle supports the following arithmetic operators:

- `+`
- `-`
- `*`
- `/`
- `%`

Example:

```rust
manle sum = a + b;
manle difference = a - b;
manle product = a * b;
manle quotient = a / b;
manle remainder = a % b;
```

## Grouping

Expressions can be grouped using parentheses:

```rust
manle result = (a + b) * c;
```

## Importing Functions

Functions can be imported from packages using the `import` and `from` keywords:

```rust
import "add" from "addition";
```

This will import the `add` function from the `addition` package.

## Printing

The `likh` keyword is used to print values:

```rust
likh "Hello, World!";
likh a;
```

## Comments

Single-line comments start with //

```rust
// This is a comment
manle a = 10; // This is another comment
```