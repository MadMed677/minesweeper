# MineSweeper Game ![Status](https://github.com/madmed677/minesweeper/actions/workflows/general.yml/badge.svg)
Minesweeper is a single-player puzzle video game.
The objective of the game is to clear a rectangular board
containing hidden "mines" or bombs without detonating any of them,
with help from clues about the number of neighbouring mines
in each field.

The game originates from the 1960s, and it has been written
for many computing platforms in use today.

[More information about the game](https://en.wikipedia.org/wiki/Minesweeper_(video_game))

## Technologies
Minesweeper was written on Rust and Typescript

### Rust
Rust contains the main logic of the game:
- Contains the main `map` (or `battlefield` inside the code)
    with provided `cols`, `rows` and provided `bombs`
- Contains the logic to reveal specific cell

### Typescript
- Contains only render part of the game. It uses `PixiJS` to render
    Canvas with WebGL and visualize board game based on Rust engine

## Develop
### How to run in debug mode
```shell
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

### How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

### How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```
