# Milestone

## Demoing in Rust Quickly

TODO:

- create debug print that includes hole numbers
- create move map
- allow user input (via CLI / position strings)

## Board Position Strings

- Creating a board from scratch:
  - "BBWxWxBB"
    - B -> black, W -> white, x -> empty
- Inputting a list of moves
  - "b6-14, w26-20, b10-t14"
  - "b6r, w18l, b10ts"
  - Main concerns are trying to make it clear who moves when (or not), if
    there's a capture on that move, spatial alignment of positions
