<img src="https://github.com/loovjo/red/blob/master/logo.png" width=200px/>

An `ed`-inspired minimalistic text editor, written in Rust.

# Usage:

## Ranges
Ranges in `red` represents a section of the lines in a document.
A range does not have to be continous, you can, for example, have a range
that includes all lines containing a specific word, etc.
A range can be made using a commands:

(`x`, `y` are other ranges, `n`, `m` are numbers)

* `n-m`: Select all lines between `n` and `m`, both inclusive.
* `n`: The `n`th line in the buffer.
* `$`: The last line in the buffer.
* `%`: The entire buffer
* `/REGEX/`: All lines matching that regex
* `x+y`: The range `x` combined with the range `y`
* `x##n`: Expand the range `x` by `n` lines
* `x^n`: The range `x`, shifted `n` lines down. (`n` can be negative)

If no range is entered, the last range is used.

## Actions
To do anything on the ranges, you can use "actions". An action can be a thing such as printing the range, deleting it, etc.
Here's a list of commands:

* `p`: Print the range with numbers
* `P`: Print the range without numbers
* `i`: Insert text before the range
* `a`: Append text after the range. End the insertion with a single period (`.`)
* `t<range>`: Copies the text in the current range to the specified location
* `s/REGEX/REPLACEMENT`: Replace `REGEX` with `REPLACEMENT` in the range.
* `c`: Change content in the middle of the line
* `w [path]`: Write the file, optionally to `[path]`
* `e <path>`: Edit that file
* `d[reg]`: Delete all the lines in that range, storing them in that register
* `y[reg]`: Copies, yanks, the lines in the range to that register
* `pa[reg]`: Pastes the contents of the specified register into the buffer at that range
* `r[reg]`: Displays the content of the specified register, or all if none specified
* `bl`: List buffers
* `bc<n>: Change to buffer `n`
* `cl`: Clears the screen
* `bn`: Create a new buffer
* `q`: Quits the current buffer. If this is the last buffer, the entire program quits.


## Registers
A register is basically a named clipboard. If no register is specified, the `'` register is defaulted.

## Examples and useful stuff:
* `%p`: Print the entire buffer, with line numbers