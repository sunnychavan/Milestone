# Milestone

## Demoing in Rust Quickly

TODO:

- refactor `move` to `can_move` and `move`
- maybe Move{} should be refactored to include origin and dest
- allow setting board state via position string from file
- test game, add logging

## Board Position Strings

- Creating a board from scratch:
  - "BBWxWxBB"
    - B -> black, W -> white, x -> empty
- Inputting a list of moves
  - "b6-14, w26-20, b10-t14"
  - "b6r, w18l, b10ts"
  - Main concerns are trying to make it clear who moves when (or not), if
    there's a capture on that move, spatial alignment of positions
