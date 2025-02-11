<p align="center">
  <a href="https://CaymanFreeman.github.io/FlappyBird"><img src="assets/icon.png" width="256" height="256" alt="Flappy Bird Logo"></a>
</p>

<div id="toc" align="center">
  <ul style="list-style: none;">
    <summary>
      <h1 align="center">
        Flappy Bird
      </h1>
    </summary>
  </ul>
</div>

<h3 align="center">
  An ECS Flappy Bird clone
</h3>

<p align="center">
  <a href="https://github.com/CaymanFreeman/FlappyBird/blob/main/LICENSE-MIT.md"><img alt="MIT License" src="https://img.shields.io/badge/license-MIT-%23B20D35?style=flat"></a>&nbsp;
  <a href="https://github.com/CaymanFreeman/FlappyBird/blob/main/LICENSE-APACHE.md"><img alt="Apache License" src="https://img.shields.io/badge/license-Apache-%23a6215a?style=flat"></a>&nbsp;
  <a href="https://www.rust-lang.org/"><img alt="Built With Rust" src="https://img.shields.io/badge/built_with-Rust-%23f74c00?style=flat"></a>&nbsp;
  <a href="https://bevyengine.org/"><img alt="Powered By Bevy" src="https://img.shields.io/badge/powered_by-Bevy-%23232326?style=flat"></a>&nbsp;
  <a href="https://www.linkedin.com/in/caymanfreeman/"><img alt="linkedin" src="https://img.shields.io/badge/linkedin-Connect_with_me-%230072b1?style=flat"></a>
</p>

## Play

You can play the game [here](https://CaymanFreeman.github.io/FlappyBird). It may take a few seconds to download the
binary and the page will appear blank while this is happening. The goal of the game is to get as far as possible while dodging the pipes. Press SPACE to flap your wings.

## Overview

This project is a clone of the original game and is based the core movement mechanics from a video
by [Biped Potato](https://www.youtube.com/watch?v=_C28kqin94c). It uses the Bevy ECS (Entity-Component-System) game
engine. Thanks to Bevy and Rust's cross-platform compatibility, the game can be compiled
for almost any platform, including WebAssembly (WASM), allowing it to be played directly in your browser. A WASM version
of the game is hosted on this repository's [gh-pages](https://github.com/CaymanFreeman/FlappyBird/tree/gh-pages) branch.

## Attributions

- Sprites from [Biped Potato](https://github.com/Biped-Potato/flappy_bird/tree/master/assets)
- Sound effects from [samuelcust](https://github.com/samuelcust/flappy-bird-assets/tree/master/audio)
- Music by [ben_burnes](https://tallbeard.itch.io/music-loop-bundle) (Sketchbook 2024-10-30)
- Title and score font, [Light Pixel-7](https://www.1001fonts.com/light-pixel-7-font.html), and button
  font, [Mini Pixel-7](https://www.1001fonts.com/mini-pixel-7-font.html), by
  Sizenko Alexander at Style-7

## Local Build

Ensure that you have installed [Git](https://git-scm.com/downloads)
and [Cargo](https://www.rust-lang.org/tools/install). Cargo and the Rust
language are bundled together in the rustup installer.

### Traditional Build

#### Clone Repository

```bash
git clone https://github.com/CaymanFreeman/FlappyBird && cd FlappyBird
```

#### Build & Run

```bash
cargo run --release 
```

### WASM Build & HTTP Server

#### Clone Repository

```bash
git clone https://github.com/CaymanFreeman/FlappyBird && cd FlappyBird
```

#### Install WASM Bindgen CLI & Basic HTML Server

```bash
cargo install wasm-bindgen-cli basic-http-server
```

#### Add Build Target

```bash
rustup target add wasm32-unknown-unknown
```

#### Build

```bash
cargo build --profile wasm-release --target wasm32-unknown-unknown
```

#### Generate JavaScript Bindings

```bash
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "flappy_bird" ./target/wasm32-unknown-unknown/wasm-release/flappy_bird.wasm
```

#### Copy Assets & HTML

```bash
cp -r assets ./out/; cp -r web/* ./out/
```

#### Start HTTP Server

```bash
basic-http-server out -a 127.0.0.1:4000
```

‎

hi :)
