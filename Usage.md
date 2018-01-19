# Ranges
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
* `x#n`: Expand the range `x` by `n` lines
* `x^n`: The range `x`, shifted `n` lines down. (`n` can be negative)

If no range is entered, the last range is used.

# Actions
To do anything on the ranges, you can use "actions". An action can be a thing such as printing the range, deleting it, etc.
Here's a list of commands:

* `p`: Print the range with numbers
* `P`: Print the range without numbers
* `i`: Insert text before the range
* `a`: Append text after the range. End the insertion with a single period (`.`)
* `s/REGEX/REPLACEMENT`: Replace `REGEX` with `REPLACEMENT` in the range.
* `c`: Change content in the middle of the line
* `w [path]`: Write the file, optionally to `[path]`
* `e <path>`: Edit that file
* `bl`: List buffers
* `bc <n>: Change to buffer `n`
* `bn`: Create a new buffer

# Examples and useful stuff:
* `%p`: Print the entire buffer, with line numbers