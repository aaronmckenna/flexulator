# Flexulator
A flexible, terminal based calculator written in Rust.
Current Version: 0.1.1

## Commands and How To Use
Hello, welcome to **Flexulator**!   
Flexulator can currently use only a limited number of operations, 
however it functions as expected with a few caveats.   

**Arguments, Flags**
| Flag | Description |
| ---- | ----------- |
| -x, --examples | Displays some examples, along with some tips of how Flexulator can be used. |
| -d, --dont | Displays what *not* to do, along with a couple of rules Flexulator currently follows. |
| -n, --new | Displays periodic updates that goes all the way back to Version 0.1.0. |
| -m, --message | Toggles the intro message |  
  
These arguments and flags can be used when Flexulator is currently
active, just without the hyphens.  

**Operators, Others**
| Operator | Usage | Description |
| -------- | ----- | ----------- |
| + | `5 + 5` | Adds the two left most numbers together. |
| - | `5 5 -` | Subtracts the two left most numbers. The second number gets subtracted from the first. |
| * | `* 5 5` | Multiplies the two left most numbers. |
| / | `/ 5 5` | Divides the left two most numbers. The second number gets divided by the first. |
| ^ | `2 ^ 10` | Uses rust's built in `powf` function. Applies to the two left most numbers, where the second number is the power. |
| ! | `! 1` | Factorial only applies to a single number. |
| % | `10 % 5` | Modulo operator, aka remainder. Essentially ten remainder five, which will be zero. |
| ln | `ln 100` | Natural log applied to the first number encountered. |
| log | `log 2 10` | Log of any base applied to the *second* number encountered. The first number is the base of the log. |
| sin, cos, tan | `sin pi` | Self explanatory |
| arcsin, arccos, arctan | `pi * 2 arctan` | Self explanatory |
| \|\| | `-2 ||` | Absolute value, however make sure not to surrond a value with the absolute value sign. It currently only works on one value at a time. |
| rt, root | `root 4 2` | The root (second number) applied to the first number. |
| sqrt | `sqrt 4` | Square root function for easier usability. |
| sqr | `sqr 2` | Square function for easier usability.

**How Flexulator Reads Inputs** 
Flexulator is somewhat strange in the way that it works, but that's what keeps it flexibile. 
This information is accurate to Version 0.1.1.  

`+ - 5 4 * * 2 10 / 9 0.5 ln` Is a lengthy example, but a lengthy example is the best way to go through it.
Flexulator starts with the first number. In this case, `5`. Then it jumps to the first operator, `+`, followed by the next number `4`.
From there, Flexulator has an interim answer and pushes it to a results stack. The stack goes to the next operator, `-`.
Then `2`, push to the stack, `* 10`, stack, `* 9`, stack, `/ 0.5`, stack, `ln`, stack. The last part may confuse you,
`ln` at the end applies to the final answer.
Flexulator works the same way that we typically read. However, in a future update (likely soon), I'd like to add a flag
that can help those who read from right to left be able to use Flexulator.  

### Installing Flexulator
Flexulator has no *official* release currently, as I'm not satisfied with it being a "tool" yet. However, if you
wish to use it, download the source, install Rust, and run `cargo run` in the same folder the source is in.  

Aaron McKenna (aaronmckenna), 2021