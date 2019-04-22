<img src="https://github.com/loovjo/red/blob/master/logo.png" width=200px/>

An `ed`-inspired, minimalistic text editor, written in Rust.

# How it looks:

`red` is a very minimalistic editor which focuses on clarity and efficiency, and therefore does not obstruct your view 
with unimportant things, such as the contents of the file you're editing. You are in full control, and you get to choose what
you want, and don't want to see.

Here is a screenshot from a typical session with `red`:
<img src="https://github.com/loovjo/red/blob/master/Screenshot.png" width=500px/>

# Usage:

## Ranges
Ranges in `red` represents a section of the lines in a document.
A range does not have to be continuous, you can, for example, have a range
that includes all lines containing a specific word, etc.
A range can be made using the following commands:

(`x`, `y` are other ranges, `n`, `m` are numbers)

* `n-m`: Select all lines between `n` and `m`, both inclusive.
* `n`: The `n`-th line in the buffer.
* `$`: The last line in the buffer.
* `%`: The entire buffer, same as `0-$`.
* `/REGEX/`: All lines matching that regex.
* `x+y`: The range `x` combined with the range `y`.
* `x*y`: The range `x` intersected with `y`, all lines within both `x` and `y`.
* `x#n`: Expand the range `x` by `n` lines.
* `x##n`: Expand the range `x` by `n` lines, upwards and downwards. Same as `(x#n)#-n`.
* `x^n`: The range `x`, shifted `n` lines down. (`n` can be negative).
* `!x`: Invert `x`, all the lines not in `x`.
* `'a`: Range in mark `a`

If no range is entered, the last range is used.

## Actions
To do anything on the ranges, you can use "actions". An action can be a thing such as printing the range, deleting it, etc.
Here's a list of commands:

(`<x>` means that `x` is mandatory, `[x]` means `x` is optional)
* `p`: Print the range with numbers.
* `P`: Print the range without numbers.
* `i`: Insert text before the range.
* `a`: Append text after the range. End the insertion with a single period (`.`).
* `t<range>`: Copies the text in the current range to the specified location.
* `A<text>`: Append `<text>` to every line in the range.
* `I<text>`: Insert `<text>` in the beginnig of every line in the range.
* `s/REGEX/REPLACEMENT`: Replace `REGEX` with `REPLACEMENT` in the range.
* `c`: Change content in the middle of the line.
* `mA`: Save the current selection into mark `A`
* `w[path]`: Write the file, optionally to `[path]`.
* `e<path>`: Edit that file.
* `d[reg]`: Delete all the lines in that range, storing them in that register.
* `y[reg]`: Copies, yanks, the lines in the range to that register.
* `pa[reg]`: Pastes the contents of the specified register into the buffer at that range.
* `r[reg]`: Displays the content of the specified register, or all if none specified.
* `bl`: List buffers.
* `bc<n>`: Change to buffer `n`.
* `bn[path]`: Create a new buffer. If `[path]` is specified, that file will be opened.
* `q`: Quits the current buffer. If this is the last buffer, the entire program quits.
* `cl`: Clears the screen.

More details about each action and range can be found in the [Details.md file](details.md)

## Registers
A register is basically a named clipboard. If no register is specified, the `'` register is defaulted.

## Marks
A mark is a saved range. They are dynamically updated so that they refer to the correct locations after adding/removing lines

## Buffers
In `red`, you can have many files open at once, called buffers. The commands `bl`, `bn`, `bc` and `q` are used to manage buffers.

## Examples and useful stuff:
* `%p`: Print the entire buffer, with line numbers.
* `0#10+$#-10p`: Print the first and last 10 lines in the buffer, with line numbers.


# Command line arguments

`red` supports a few flags:

* `-s`: Turn on silent mode. This makes many commands not show any information unless crutial. For example, `e` usually shows the text "Editing ...", but this is suppressed with `-s`.
* `-d <commands>`: Using this flag you can supply commands to execute from the command line. `<commands>` is separated by semicolons (`;`), which can be escaped with `\`.
When using this, `red` won't read any input from STDIN and will discard the buffer when there are no commands left.
* `--`: Read the buffer from STDIN. This can be usefull when processing output from other utilities in the command line, combined with `-d`. This flag on it's own is not very useful as you
can't enter any commands with it.