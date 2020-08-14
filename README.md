Rust Wasm Sokoban
---

This project is try to build a sokoban game with wasm, it is a self-learning & share project.

Here are my objectives
  - Know more about WASM
  - Have ability to build UI with Rust
  - Share knowlege and help Rust ecosystem

If you are also interesting in this, it is very welcome to create issue or PR here, or click star of this project.


### Notes

Following is some material about this topic
- [Wasm Pack Template](https://github.com/rustwasm/wasm-pack-template)
  - a project helps programer to initialize wasm project
- [Rust Sokoban](https://github.com/iolivia/rust-sokoban)
  - a project helps you to use ggez(not wasm) to build a sokoban game
- [ggez](https://github.com/ggez/ggez)
  - a 2d game framework (not support wasm)
- [Good Web Game](https://github.com/not-fl3/good-web-game)
  - a wasm 2d game framework (subset of ggez)
  - on top of [miniquad](https://github.com/not-fl3/miniquad)
    - miniquad to build up uniform platform in `native`
    - sapp-wasm build `gl.js` as a glue script
    - [samples](https://not-fl3.github.io/miniquad-samples/)
- [specs](https://github.com/amethyst/specs)
  - an entity-component system (parallel)
- [Zemeroth](https://github.com/ozkriff/zemeroth)
  - a game based on good web game, but not use ECS
