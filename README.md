# RS - A 'ls' alternative for unix systems made in Rust
<a href="https://gitmoji.carloscuesta.me">
  <img src="https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square" alt="Gitmoji">
</a>

---

## What it intends to be?
A zero-dependency Rust alternative to unix ls. This project is based on existing projects, specially [lsd](https://github.com/Peltoche/lsd) and [Exa](https://github.com/ogham/exa) as ls alternatives, and [clap](https://github.com/clap-rs/clap) as the cli control.

## What it is now?
Right now it can list files as simple as possible and has a base for a possibly robust cli interface based on [clap](https://github.com/clap-rs/clap). Unfortunately, now there is a single dependency, libc, because I need to get console dimensions (and maybe other stuffs later on) and didn't find a better way to do it without ioctl. You can find the roadmap on [this issue](https://github.com/jansen44/rs/issues/1), this is by no means the final roadmap, I just put ideas there as I code. Many items can be added and some items can be removed as I see them viable or not.
