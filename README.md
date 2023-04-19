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

## Arguments

- Passing any arguments to this program (ie `cargo run genetic` will cause the
  program to launch in genetic mode)
- In addition, the following env vars can be set:
  - `LAUNCH_ARG`: corresponds to the number input at the beginning
  - `PER_NUM_BATCHES`, `TOTAL_NUM_BATCHES`, `NUM_AGENTS_RETAINED`,
    `NUM_CHILDREN_PER_RETAINED_AGENT`, `MAX_PERTURB_AMT`, `PERTURB_DECR`,
    `NUM_AGENTS`, `NUM_MATCHES`, `AGENT_DEPTH`
  - `PLAY_AFTER` to toggle if a game is launched following the completion of the
    genetic process (currently the program only checks if this var exists, not
    its value)
  - `DATABASE_URL` for the database URL

## Running on Server

This process can be launched via `./target/release/milestone` (after compiling),
which can be done periodically using a cron job. For example, we can relaunch
this process every two hours, and the recovery database will ensure that process
is maintained. Logs will persist in the `logs/` directory and stderr can be
piped to a file to debug any other issues with the process.

## Database

- Download VsCode Sqlite Extension (SQLite by alexcvzz)
