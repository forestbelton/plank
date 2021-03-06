![plank](https://i.imgur.com/cGfTu.jpg)

plank [![Build Status](https://travis-ci.org/forestbelton/plank.svg?branch=master)](https://travis-ci.org/forestbelton/plank)
=====
A lightweight message board in Rust

Installation
------------

If [Docker](https://www.docker.com/) is available, you can build a container with the following steps from the top-level directory of the project:

```
$ docker build -t plank .
$ docker run --rm --name plank -p3000:3000 -it plank
```

Otherwise, make sure [Cargo](https://crates.io/) and `libsqlite3` development libraries are present on your system and perform a `cargo build`. For example, on OSX:

```
$ brew install sqlite
$ curl https://sh.rustup.rs -sSf | sh
...
$ cargo build
```

On Ubuntu systems, you will need the `libsqlite3-dev` package to build the application and the `libsqlite3-0` package to run it. Once you have built the application, you can start it by executing `cargo run`.

License
-------

```
Copyright (c) 2017 Forest Belton

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
