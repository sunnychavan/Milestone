# Milestone

We developed a game-playing AI for a novel board game called Milestone,
developed by a Cornell Math Ph.D. student Mark Schachner. We've implemented this
AI two-fold: as a tree-search-based, hyper-heuristic AI that is improved
genetically; and as a NN trained on the resulting data.

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
