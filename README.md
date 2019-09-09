# singularity-rs
[![Build Status](https://travis-ci.com/singularity-rs/singularity-rs.svg?branch=master)](https://travis-ci.com/singularity-rs/singularity-rs)
[![GPLv3 license](https://img.shields.io/badge/License-GPLv3-blue.svg)](http://perso.crans.org/besson/LICENSE.html)

This will be a reimplementation of the game Singularity ( https://github.com/SoPra18-07/Singularity ) in Rust using [amethyst](https://amethyst.rs/). Also inspired from [Achikaps](https://play.google.com/store/apps/details?id=yio.tro.achikaps).

Stage: in planning.


## Modules that are still needed (or things I need to think through)
(and are somewhat on their own, so they might result in their own crates if nothing applicable can be found)

<!-- - [ ] Units/Sprites (including health-system, spawn till death) -->
- [x] Screen Manager -> not necessary, use amethyst state machine
- [x] Input Manager -> not necessary, use amethyst event channels
- [x] Sound Manager -> there is something in amethyst
- [ ] Physics / Game mechanics
- [ ] Map
- [ ] UI (might depend on InputMgr)
- [ ] Pathfinding (`graph-paths`, `flow-paths`, `heatmap`, `A*`, ...)
- [ ] Prioritized Task-based work distribution (DistributionManager)
- [ ] Platforms (with all their mechanics, PlatformActions) (will probably result in a combination of entities and systems and event channels)
- [ ] The Graphics themselves (get them somewhere or create own?)
- [ ] Seperate Utils (?)



## things to keep in mind (for myself)

- decouple graphics itself quite intensely from everything else to make it simple to make a 3D-version as well
- Make fixed steps of 45° and lengths of [1, 2, 3] * fixed_length
- [ ] Plan it all out before writing code. Update as needed.
- [ ] Do the Amethyst tutorial.




## How to run

To run the game, simply use `cargo run`.


On macOS: `cargo run --no-default-features --features "metal"`.

