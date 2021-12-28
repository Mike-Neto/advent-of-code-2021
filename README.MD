# Advent of code 2021

https://adventofcode.com/2021

# Run all the things

    cargo fmt && cargo clippy && cargo b --release && cargo t --release && cargo r --release

# Deps

## bit-vec

Useful but I still needed to implement to_uint methods which is kind of dumb, maybe there is a spot here for a new lib that makes working with bits as numbers & vectors easier

## itermore

Kind of unnecessary as I only realized that `Vec` and not `Iterator` implement the `chunks`/`window` methods


# Notes

# &str split

It's kind of annoying its default behavior, I would expect there be (just like in js) a trimmed_split / join functions
