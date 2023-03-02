# Milestone

## Demoing in Rust Quickly

TODO:

- AI will often defer winning in favor of making a different move (because at a
  given depth, it will still eventually win), putting it off for very long
  periods of time
- enable the player (at the CLI) an option to print the evaluation tree from the
  previous move
- allow setting board state via position string from file
- test game, add logging
- fix print/logs from the AI's move
  - include information about the tree that was constructed?
- formalize heuristic interface, make combined heuristic builder
- measure performance impact of other forms of building the tree (storing moves
  made and rolling them forward/back)
- add a/b pruning

## Board Position Strings

- Creating a board from scratch:
  - "BBWxWxBB"
    - B -> black, W -> white, x -> empty
- Inputting a list of moves
  - "b6-14, w26-20, b10-t14"
  - "b6r, w18l, b10ts"
  - Main concerns are trying to make it clear who moves when (or not), if
    there's a capture on that move, spatial alignment of positions
