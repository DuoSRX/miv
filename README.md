# MIV - A text editor

A simple text editor inspired by vim.
Built in Rust. Just for fun!

## Installation

```
$ cargo build --release
```

## Usage

```
$ target/release/miv
```

The keybindings are vi-style, with some Spacemacs inspired ones.

* `h` `j` `k` `l` to move left, down, up and right respectively.
* `$` `0` beginning and end of line
* `gg` `GG` beginning and end of file
* `x` delete a single character
* `p` paste yanked content
* `yy` yank a line
* `dd` delete a line
* `A` `I` Move to the beginning/end of the line and switch to insert mode
* `a` `i` switch to Insert mode
* `R` switch to Replace mode
* `:q` quit
* `:w` save
* `:wq` save and quit
* `:new` make a new empty buffer
* `e file.txt` open the given file in a new buffer
* `SPC bp` `SPC bn` previous and next buffer

Like Vim, most commands can be repeat a number of time by prefixing them with a number. For instance, `15dd` will delete 15 lines.

## Caveats

* Handles only `LF` line ending. It will react weirdly with other types.
