# Codingame

This repository contains my solutions to various Codingame puzzles. If you're also using Codingame to learn or polish some skills I recommend attempting the puzzles blind before looking at someone else's solution!

For these solutions I will prioritize code readability, measured only by my subjective opinion. For most puzzles I will be discarding the starter/default code entirely. I do this partially for my own desire to start as much from scratch as possible, though it's also in part in response to some of the default code provided. As a specific example of what I mean, all of the default Rust code for puzzles I've seen so far contains input parsing macros such as:

```rust
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
```

I can tell what this macro does but I'm not yet very savvy with macros in Rust, so I would not have thought of abstracting user input parsing in this same way. Later I may revisit this kind of code and find myself much fonder of it, but for now I want the entirety of my solutions to be composed of code I have written.

## Solutions

### Easy Puzzles

- Onboarding