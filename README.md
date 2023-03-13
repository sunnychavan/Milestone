# Milestone

## Demoing in Rust Quickly

TODO:

- AI will often defer winning in favor of making a different move (because at a
  given depth, it will still eventually win), putting it off for very long
  periods of time
- enable the player (at the CLI) an option to print the evaluation tree from the
  previous move
- test game, add logging
- measure performance impact of other forms of building the tree (storing moves
  made and rolling them forward/back)

## Board Position Strings

- Creating a board from scratch:
  - "b:b/bb/bbb/bbbb/3/4/3/4/3/wwww/www/ww/w"
    - `b` -> black, `b` -> white, `x` -> number of blank squares, `/` means next
      line, `b:` means black's turn (`w:` for white's turn)
