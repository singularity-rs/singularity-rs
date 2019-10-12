# singularity-rs
[![Build Status](https://travis-ci.com/singularity-rs/singularity-rs.svg?branch=master)](https://travis-ci.com/singularity-rs/singularity-rs)
[![codecov](https://codecov.io/gh/singularity-rs/singularity-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/singularity-rs/singularity-rs)
[![Docs](https://img.shields.io/badge/docs-here-brightgreen)](https://singularity-rs.github.io/singularity-rs/)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=singularity-rs/singularity-rs)](https://dependabot.com)
[![Bors enabled](https://bors.tech/images/badge_small.svg)](https://app.bors.tech/repositories/20537)
[![GPLv3 license](https://img.shields.io/badge/License-GPLv3-blue.svg)](http://perso.crans.org/besson/LICENSE.html)
<!-- [![GitHub tag](https://img.shields.io/github/v/tag/singularity-rs/singularity-rs.svg)](https://github.com/singularity-rs/singularity-rs/tags/) -->

This will be a reimplementation of the game Singularity ( https://github.com/SoPra18-07/Singularity ) in Rust using [amethyst](https://amethyst.rs/). Also inspired from [Achikaps](https://play.google.com/store/apps/details?id=yio.tro.achikaps).

Stage: implementing parts / planning.


## How to run

To run the game, simply use `cargo run`. A recent rust installation is recommended. Make sure to have the [amethyst dependencies](#amethyst-dependencies) installed.


On macOS: `cargo run --no-default-features --features "metal"`.


## Modules that are still needed (or things I need to think through)
(and are somewhat on their own, so they might result in their own crates if nothing applicable can be found)

<!-- - [ ] Units/Sprites (including health-system, spawn till death) -->
- [x] Screen Manager -> use amethyst state machine
- [x] Input Manager -> amethyst has one
- [x] Sound Manager -> amethyst has one
- [ ] Physics / Game mechanics -> keep looking
- [ ] Map -> might need custom implementation
- [x] UI -> amethyst has considerable implementations already. There is little documentation though ...
- [ ] Pathfinding (`graph-paths`, `flow-paths`, `heatmap`, `A*`, ...) -> probably custom implementation
- [ ] Prioritized Task-based work distribution (DistributionManager) -> absolutely custom implementation
- [ ] Platforms (with all their mechanics, PlatformActions) (will probably result in a combination of entities and systems and event channels) -> custom implementation
- [ ] The Graphics themselves -> get them somewhere or create own?
<!-- - [ ] Seperate Utils (?) -->



## Progress

A plan of things I intend to implement (which is one of my initial plans for this project) can be found in the [PLAN.md](https://github.com/singularity-rs/singularity-rs/blob/master/PLAN.md).

Take a look at the [issue tracker](https://github.com/singularity-rs/singularity-rs/issues) or the [Project](https://github.com/singularity-rs/singularity-rs/projects/1?fullscreen=true) how the progress is in detail (it is likely to be more frequently updated than the plan). If something is not clear, feel free to ask for clarification.



## People to thank

The awesome Logo was made by Yvan Satyawan [yvan674](https://github.com/yvan674) (Splash.jpeg and Splash2.jpg).


## Amethyst Dependencies

If you are compiling on Linux, make sure to install the dependencies below.

### Arch Linux

```
$ pacman -Sy grep gcc pkgconfig openssl alsa-lib cmake make python3 freetype2 awk libxcb
```

### Debian/Ubuntu

```
# apt install gcc pkg-config openssl libasound2-dev cmake build-essential python3 libfreetype6-dev libexpat1-dev libxcb-composite0-dev libssl-dev libx11-dev
```

### Fedora

```
# dnf install pkgconfig gcc openssl-devel alsa-lib-devel cmake make gcc-c++ freetype-devel expat-devel libxcb-devel libX11-devel
```

### openSUSE

```
# zypper install gcc pkg-config libopenssl-devel alsa-devel cmake gcc-c++ python3 freetype2-devel libexpat-devel libxcb-devel
```

### Other

See your distribution-specific installation process for the equivalent dependencies.

**Please note that you need to have a functional graphics driver installed. If you get a panic about the renderer unable to create the context when trying to run an example, a faulty driver installation could be the issue.**


## Other

Network can be found [here](https://github.com/singularity-rs/singularity-rs/network).


