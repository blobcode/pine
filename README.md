# 🌲 pine

- [About](#about)
- [Installing](#installing)
- [Development](#development)
- [Usage](#usage)

## About <a name = "about"></a>

Pine is a simple rust-based reverse proxy built on top of hyper that supports multiple hosts and has super simple configuration!

Please note that this should probably not be used in favor of [cobalt](https://github.com/blobcode/cobalt), which is a better solution on many different levels.

## Installing <a name = "installing"></a>

To install the current version normally, run
```
cargo install --git https://github.com/blobcode/pine
```

This assumes that you have both cargo and git installed.

For next steps, take a look in [usage](#usage).


## Development <a name = "development"></a>

To install for development, clone this git repo and `cd` into it

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

Pine relies on a configuration file named `pine.toml` in the current working directory or one specified using `pine <path to pine.toml>`. An example `pine.toml` is provided below.

```
port = 8080

[[host]]
from = ["localhost:8080", "example.com"]
to = "localhost:4000"
```

Let's take a look at `[[host]]`. It represents an internal http service you want forwarded externally. In our case, it denotes that we should forward all requests looking for `localhost:8080` or `example.com` to `localhost:4000`.

### Load Balancing
If you want to include load balancing, take a look at another project of mine, [scarf](https://github.com/blobcode/scarf), a super simple tcp load balancer written in rust.
