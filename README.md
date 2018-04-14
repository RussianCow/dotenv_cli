# dotenv_cli

A zero-dependency command line tool, written in Rust, that loads environment variables from a  `.env` file before running another command.

### Installation

An official installation path is coming soon, along with release binaries for macOS and Linux. (Windows support will come a little bit later.) In the meantime, you can clone this repo and build using [Cargo](https://doc.rust-lang.org/cargo/):

```
$ cargo build --release
```

You can then use the binary under `target/release` (and optionally rename it `dotenv`).

### Usage

Create a `.env` file in the current working directory that contains environment variable key/value pairs. Something like this:

```
KEY1=some-value
KEY2=something-else
```

Then run your program via dotenv_cli:

```
$ dotenv your-program arg1 arg2 ...
```

### .env file format

Each line of a `.env` file must contain one of the following:

* Nothing (will be ignored)
* A comment starting with `#`
* A key/value pair, separated by an equals sign (like  `KEY=value`)

Keys must only contain uppercase, alphanumeric ASCII characters, and cannot start with a number. Any whitespace found after the equals sign will be treated as part of the value.

### Roadmap

This is a very early release, and not intented to be used in production yet. Major todo items (roughly in order):

- [ ] Add an optional argument for specifying the path to the `.env` file you would like to use
- [ ] Add binary releases and introduce an official installation path for each platform
- [ ] Add tests
- [ ] Add Windows support