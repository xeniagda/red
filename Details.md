

# Ranges

## True ranges and line ranges
This is a line containing an error which needs to be changed

When writing ranges, there are normal ranges and a subset called "line ranges".
Line ranges are, as the name suggests, ranges that just span one line. An example of this is `$`.
Line ranges can be used in exactly the same way as normal ranges, but support a few extra features.
You can do `a-b` where `a` and `b` are line ranges to create a normal range that goes from `a` to `b`, inclusive.
Commands such as `^` will keep the type of range, so eg. `$^10` will still be considered a line range.

## Advanced range construction details

* `x#n`: If `n` is positive, `x` is copied downwards onto itself `n` times, otherwise `x` is copied upwards `-n` times. For example, `5#7` will select lines 5, 6, 7
and `/Hello/#3` will span every line containing the word "Hello" and three lines below it.


# Actions

* `a`, `i`: These commands insert text at the respective locations. `i` inserts text before and `a` inserts, or appends, text after the range.
If the range spans multiple lines, this command will insert/append text at every selected line.
To end inserting text, enter a line containing nothing but a single period: `.`

* `c`: This command is used to change the content of a line.
The line is displayed, together with "targets" below each char. Each target is just a unique chararcter which you can select the range you want to change.
You enter start and finish, two chars with no seperator between. You will then be prompted with what you want to exchange that range with.
Here's an example of it's usage:

    5c
      This is a line containing an errror which needs to be changed
      0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ
    T> ty
    c> error
    p
    5 This is a line containing an error which needs to be changed