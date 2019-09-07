[![Build Status](https://travis-ci.com/singularity-rs/singularity-rs.svg?branch=master)](https://travis-ci.com/singularity-rs/singularity-rs)

# singularity-rs
This will be a reimplementation of the game Singularity ( https://github.com/SoPra18-07/Singularity ) in Rust using [amethyst](https://amethyst.rs/).

Stage: heavily in planning.


## Components that are still needed
(and are somewhat on their own, so they might result in their own crates if nothing applicable can be found)

<!-- - [ ] Units/Sprites (including health-system, spawn till death) -->
- [ ] Screen Manager
- [ ] Input Manager
- [ ] Sound Manager
- [ ] Physics / Game mechanics
- [ ] Map
- [ ] UI (might depend on InputMgr)
- [ ] Pathfinding (`graph-paths`, `flow-paths`, `heatmap`, `A*`, ...)
- [ ] Prioritized Task-based work distribution
- [ ] Platforms (with all their mechanics, PlatformActions)
- [ ] The Graphics themselves (get them somewhere or create own)
- [ ] Seperate Utils (?)



## things to keep in mind (for myself)

- decouple graphics itself quite intensely from everything else to make it simple to make a 3D-version as well
- Make fixed steps of 45° and lengths of [1, 2, 3] * fixed_length
- [ ] Plan it all out before writing code. Update as needed.
- [ ] Do the Amethyst tutorial.




## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.
