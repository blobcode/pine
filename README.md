# 🌲 pine

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)

## About <a name = "about"></a>

Pine is a simple rust-based reverse proxy built on top of hyper.

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.
### Prerequisites

You will need an installed version of rust to build the project.

### Installing

To install, clone this git repo and `cd` into it

```
git clone https://github.com/blobcode/pine
```
```
cd pine
```
and run it using cargo.
```
cargo run
```
For best performance, run using the release flag enabled.
```
cargo run --release
```

## Usage <a name = "usage"></a>

Pine relies on a configuration file named `config.ini` in the executable's directory. An example is provided below.
```
[config]
port = 3000
hosts = a, b

[a]
from = a.com
to = localhost:4000

[b]
from = b.com
to = localhost:5000
```
Here we can see the `[config]` header contains `port` and `hosts`. Port is the local port it will run on, and hosts is a list of the headers of all hosts you want served from the ones below.

Now let's take a look at `[a]` and `[b]`. They represent internal http services you want forwarded externally. In `[a]`'s case, it denotes that we should forward all requests looking for `a.com` to `localhost:4000`.