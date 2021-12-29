# Advent of code 2021

https://adventofcode.com/2021

# Run all the things

    // To use when dev
    cargo fmt && cargo clippy && cargo b --release && cargo t --release && cargo r --release
    // To use before commit
    cargo fmt && cargo clippy --all-features --all --tests -- -W clippy::all -W clippy::style -W clippy::complexity -W clippy::suspicious -W clippy::pedantic -W clippy::nursery -W clippy::perf -D warnings && cargo b --release && cargo t --release && cargo r --release

# Deps

## bit-vec

Useful but I still needed to implement to_uint methods which is kind of dumb, maybe there is a spot here for a new lib that makes working with bits as numbers & vectors easier

## itermore

Kind of unnecessary as I only realized that `Vec` and not `Iterator` implement the `chunks`/`window` methods


## array_tool

Relates with my comment bellow as it has some useful functions for working with vectors & strings (intersect, uniq & join) to be specific.

# Notes

## &str split

It's kind of annoying its default behavior, I would expect there be (just like in js) a trimmed_split / join functions

## Day_06 lanternfish

Would be nice to make a precomputed cache and load from disk just so we can run all things in CI. Otherwise this day takes too long to complete.

## Vec<Vec<Number>>

Kind of wish for a crate that would give me a simple layer on top of that and allowed for some more unusual iterators, like perimeter or cross and would bounds check tem and all of that.

## Num types

50% of what I do is fighting the type system about the precision & overflow of numbers, kind of wish I could do away with it in some form (always use f32 or something)