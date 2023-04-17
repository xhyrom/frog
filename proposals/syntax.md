# Proposal for a new syntax for the language

## The syntax is based on the following principles:

1.  The syntax should be as simple as possible

2.  The syntax should be as regular as possible

3.  The syntax should be as unambiguous as possible

4.  The syntax should be as readable as possible

5.  The syntax should be as concise as possible

6.  The syntax should be as consistent as possible

7.  The syntax should be as extensible as possible

8.  The syntax should be as familiar as possible

9.  The syntax should be as easy to learn as possible

10. The syntax should be as easy to parse as possible

## Examples of the syntax are given in the examples section.

### Variables

```
# immutable variable
val x: Int = 1

# mutable variable
var y: Int = 2
```

### Functions

```
# function with no arguments
fn f(): Int = 1;

# function with one argument
fn f(x: Int): Int = x;

# function with multiple arguments
fn f(x: Int, y: Int): Int = x + y;

# function with multiple arguments and multiple return values
fn f(x: Int, y: Int): (Int, Int) = (x, y);

# function with body
fn f(x: Int, y: Int): Int {
  return x + y;
}
```

### Comments

```
// comment

/**
 * multi-line comment
 */
```
