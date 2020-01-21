# rust-pushrod

## Project Description

[![Build Status](https://travis-ci.org/KenSuenobu/rust-pushrod.svg?branch=master)](https://travis-ci.org/KenSuenobu/rust-pushrod)
[![](https://img.shields.io/crates/d/rust-pushrod.svg)](https://crates.io/crates/rust-pushrod)
[![docs.rs for rust-pushrod](https://docs.rs/rust-pushrod/badge.svg)](https://docs.rs/rust-pushrod)

**Cross Platform UI Widget Library for Rust that uses SDL2.**

Draws inspiration from lots of GUI libraries.

If you like this library, [please consider donating to this project!](https://www.patreon.com/KenSuenobu)

## Philosophy

The reason I created this library instead of extending another library was that
I wanted to keep these specific design ideas in mind:

- Maintainable with little effort
- Easily extensible
- Lightweight enough to run on minimalist hardware
- **Easy to use and understand**

These design ideas are critical.  **Keep it simple.  Keep it stupid simple.**

[Click here to view my Blog!](https://kensuenobu.github.io/)

## 0.4.x Status

Please [see here](https://github.com/KenSuenobu/rust-pushrod/milestone/5) for more details on issues.

## Prerequisites for Pushrod

Pushrod only requires:

| Library | Version |
| ------- | ------- |
| SDL2    | 0.32 |

### Ubuntu

```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev
```

### Mac OS X

```bash
brew update
brew upgrade
brew install ruby
brew install sdl2 sdl2_image sdl2_ttf
```
