<div align="center">

  <a href="https://lospec.com/gallery/orhun/octopus">
    <img src="assets/halp-logo.png" width="400">
  </a>

<h4>A CLI tool to get help with CLI tools 🐙</h4>

<a href="https://github.com/orhun/halp/releases"><img src="https://img.shields.io/github/v/release/orhun/halp?style=flat&amp;labelColor=342a5e&amp;color=684d81&amp;logo=GitHub&amp;logoColor=white" alt="GitHub Release"></a>
<a href="https://crates.io/crates/halp/"><img src="https://img.shields.io/crates/v/halp?style=flat&amp;labelColor=342a5e&amp;color=684d81&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://codecov.io/gh/orhun/halp"><img src="https://img.shields.io/codecov/c/gh/orhun/halp?style=flat&amp;labelColor=342a5e&amp;color=684d81&amp;logo=Codecov&amp;logoColor=white" alt="Coverage"></a>
<br>
<a href="https://github.com/orhun/halp/actions?query=workflow%3A%22Continuous+Integration%22"><img src="https://img.shields.io/github/actions/workflow/status/orhun/halp/ci.yml?branch=main&amp;style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=GitHub%20Actions&amp;logoColor=white" alt="Continuous Integration"></a>
<a href="https://github.com/orhun/halp/actions?query=workflow%3A%22Continuous+Deployment%22"><img src="https://img.shields.io/github/actions/workflow/status/orhun/halp/cd.yml?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=deploy" alt="Continuous Deployment"></a>
<a href="https://hub.docker.com/r/orhunp/halp"><img src="https://img.shields.io/github/actions/workflow/status/orhun/halp/docker.yml?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;label=docker&amp;logo=Docker&amp;logoColor=white" alt="Docker Builds"></a>
<a href="https://docs.rs/halp/"><img src="https://img.shields.io/docsrs/halp?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=Rust&amp;logoColor=white" alt="Documentation"></a>

<img src="./assets/halp-demo.gif" alt="halp demo">

</div>

`halp` aims to help find the **correct arguments** for command-line tools by checking the predefined list of commonly used options/flags. Additionally, it provides a prompt for quick access to the **manual page** or **cheat sheet** of the given command.

If you deal with command-line tools often, it might take some time to figure out how to get help or check the version of a particular command (especially when shell completions are not available). In that case, you might try the most-known flags such as `-h` and `-v` but unfortunately not all the command-line tools follow these conventions (either due to conflicts with other flags or they just use another form). Instead of _brute-forcing_ manually into getting help, you can run `halp <command>` and it will check the following arguments for you:

- for **help**: `-h`, `--help`, `help`, `-H`
- for **version info**: `-v`, `-V`, `--version`, `version`

If one of these arguments succeeds (with exit code 0), it prints the output and exits. This way, you can get informed about the version and help in one single command. You can also customize this list with a configuration file or provide a list of arguments via command-line arguments.

On the other hand, if you _really_ need help, you can use the `plz` subcommand which will prompt a selection for:

1. show the **man page** (runs [`man(1)`](https://man7.org/linux/man-pages/man1/man.1.html))
2. show the **cheat sheet** (via [`cheat.sh`](http://cheat.sh))

<details>
  <summary>Table of Contents</summary>

<!-- vim-markdown-toc GFM -->

- [Example](#example)
- [Installation](#installation)
  - [Cargo](#cargo)
  - [Arch Linux](#arch-linux)
  - [Docker](#docker)
    - [Images](#images)
    - [Usage](#usage)
    - [Building](#building)
  - [Binary releases](#binary-releases)
  - [Build from source](#build-from-source)
- [Usage](#usage-1)
  - [`plz`](#plz)
- [Examples](#examples)
  - [Check `help` and `version` (default)](#check-help-and-version-default)
  - [Check a custom argument](#check-a-custom-argument)
  - [Disable defaults](#disable-defaults)
  - [Verbose logging](#verbose-logging)
  - [Get additional help (via `plz`)](#get-additional-help-via-plz)
    - [Custom pager](#custom-pager)
    - [Custom cheat.sh host URL](#custom-cheatsh-host-url)
- [Configuration](#configuration)
- [Funding](#funding)
- [Contributing](#contributing)
- [License](#license)
- [Copyright](#copyright)

<!-- vim-markdown-toc -->

</details>

### Example

Have you ever experienced this:

```sh
$ cli_tool -v
unknown flag -v
```

```sh
$ cli_tool -V
unknown flag -V
```

```sh
$ cli_tool -h
unknown flag -h
```

```sh
$ asdjw1jwhdajh1idojad # frustration
bash: asdjw1jwhdajh1idojad: command not found
```

```sh
$ cli_tool --help # f*cking finally!
Some CLI Tool Version 1.42.69
Usage:
  cli_tool <flags> <args> [--parameter1 value1 --parameter2 value2 ...]
```

Whereas with `halp`:

```
$ halp cli_tool

(°ロ°)  checking 'cli_tool -v'
(×﹏×)      fail '-v' argument not found.
(°ロ°)  checking 'cli_tool -V'
(×﹏×)      fail '-V' argument not found.
(°ロ°)  checking 'cli_tool -h'
(×﹏×)      fail '-h' argument not found.
(°ロ°)  checking 'cli_tool --help'
\(^ヮ^)/ success '--help' argument found!

Some CLI Tool Version 1.42.69
Usage:
  cli_tool <flags> <args> [--parameter1 value1 --parameter2 value2 ...]
```

## Installation

<details>
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/halp.svg)](https://repology.org/project/halp/versions)

</details>

### Cargo

`halp` can be installed from [crates.io](https://crates.io/crates/halp):

```sh
cargo install halp
```

The minimum supported Rust version is `1.74.1`.

### Arch Linux

`halp` can be installed from the [community repository](https://archlinux.org/packages/community/x86_64/halp/) using [pacman](https://wiki.archlinux.org/title/Pacman):

```sh
pacman -S halp
```

Or you can install the available [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=halp&outdated=&SB=n&SO=a&PP=50&do_Search=Go) using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example,

```sh
paru -S halp-git
```

Alternatively, you can clone the AUR package and then build it with [makepkg](https://wiki.archlinux.org/index.php/Makepkg). For example,

```sh
git clone https://aur.archlinux.org/halp-git.git && cd halp-git && makepkg -si
```

### Docker

#### Images

Docker builds are [automated](./.github/workflows/docker.yml) and images are available in the following registries:

- [Docker Hub](https://hub.docker.com/r/orhunp/halp)
- [GitHub Container Registry](https://github.com/orhun/halp/pkgs/container/halp)

#### Usage

The following commands can be used to get help for a binary inside the container:

```sh
docker run --rm -it "orhunp/halp:${TAG:-latest}" whoami
docker run --rm -it "orhunp/halp:${TAG:-latest}" plz whoami
```

Or you can provide a custom binary as follows (please note that you might get shared library errors):

```sh
docker run -v "bin:/app/bin:rw" --rm -it "orhunp/halp:${TAG:-latest}" -v ./bin
```

#### Building

Custom Docker images can be built from the [Dockerfile](./Dockerfile):

```sh
docker build -t halp .
```

### Binary releases

See the available binaries for different targets from the [releases page](https://github.com/orhun/halp/releases). They are automated via [Continuous Deployment](.github/workflows/cd.yml) workflow

Release tarballs are signed with the following PGP key: [0xFB41AE0358378256](https://keyserver.ubuntu.com/pks/lookup?search=0xFB41AE0358378256&op=vindex)

### Build from source

1. Clone the repository.

```sh
git clone https://github.com/orhun/halp && cd halp/
```

2. Build.

```sh
CARGO_TARGET_DIR=target cargo build --release
```

Binary will be located at `target/release/halp`.

## Usage

```
halp [OPTIONS] <CMD>
```

```
Options:
      --check <ARG>    Sets the argument to check
      --no-version     Disable checking the version information
      --no-help        Disable checking the help information
  -c, --config <PATH>  Sets the configuration file [env: HALP_CONFIG=]
  -t, --timeout <S>    Sets the timeout for the command [default: 5]
  -v, --verbose        Enables verbose logging
  -h, --help           Print help
  -V, --version        Print version
```

### `plz`

```
halp [OPTIONS] plz <CMD>
```

```
Options:
  -m, --man-cmd <MAN_CMD>   Sets the manual page command to run
      --cheat-sh-url <URL>  Use a custom URL for cheat.sh [env: CHEAT_SH_URL=]
  -p, --pager <PAGER>       Sets the pager to use
      --no-pager            Disables the pager
  -h, --help                Print help
```

## Examples

#### Check `help` and `version` (default)

```sh
halp whoami
```

![halp example I](./assets/halp-example1.gif)

#### Check a custom argument

```sh
halp --check "\--silent" zps
```

(You can escape `-` with using `\-`.)

You can also provide multiple arguments as follows:

```sh
halp --check "help" --check "test" menyoki
```

#### Disable defaults

```sh
halp --no-version sha512sum
```

```sh
halp --no-help sha512sum
```

#### Verbose logging

```sh
halp --verbose git-cliff
```

This will result in `stderr`/`stdout` being printed if there was an error. For example:

```sh
(°ロ°)  checking 'git-cliff -v'
(×﹏×)      fail '-v' argument not found.
(o_O)      debug
stdout:
 WARN  git_cliff > "cliff.toml" is not found, using the default configuration.
 ERROR git_cliff > Git error: `could not find repository from '.'; class=Repository (6); code=NotFound (-3)`
```

#### Get additional help (via `plz`)

```sh
halp plz vim
```

![halp example II](./assets/halp-example2.gif)

##### Custom pager

```sh
halp plz --pager bat vim
```

To disable the pager:

```sh
halp plz --no-pager bat vim
```

##### Custom cheat.sh host URL

```sh
halp plz --cheat-sh-url https://cht.sh vim
```

## Configuration

`halp` can be configured with a configuration file that uses the [TOML](https://en.wikipedia.org/wiki/TOML) format. It can be specified via `--config` or `HALP_CONFIG` environment variable. It can also be placed in one of the following global locations:

- `<config_dir>` `/` `halp.toml`
- `<config_dir>` `/` `halp/halp.toml`
- `<config_dir>` `/` `halp/config`

`<config_dir>` depends on the platform as shown in the following table:

| Platform | Value                                 | Example                                  |
| -------- | ------------------------------------- | ---------------------------------------- |
| Linux    | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/orhun/.config                      |
| macOS    | `$HOME`/Library/Application Support   | /Users/Orhun/Library/Application Support |
| Windows  | `{FOLDERID_RoamingAppData}`           | C:\Users\Orhun\AppData\Roaming           |

See [halp.toml](config/halp.toml) for the default configuration values.

## Funding

If you find `halp` and/or other projects on my [GitHub profile](https://github.com/orhun/) useful, consider supporting me on [GitHub Sponsors](https://github.com/sponsors/orhun) or [becoming a patron](https://www.patreon.com/join/orhunp)!

[![Support me on GitHub Sponsors](https://img.shields.io/github/sponsors/orhun?style=flat&logo=GitHub&labelColor=342a5e&color=684d81&logoColor=white)](https://github.com/sponsors/orhun)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpatrons&style=flat&logo=Patreon&labelColor=342a5e&color=684d81&logoColor=white)](https://patreon.com/join/orhunp)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpledges&style=flat&logo=Patreon&labelColor=342a5e&color=684d81&logoColor=white&label=)](https://patreon.com/join/orhunp)

## Contributing

See our [Contribution Guide](./CONTRIBUTING.md) and please follow the [Code of Conduct](./CODE_OF_CONDUCT.md) in all your interactions with the project.

## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2023-2024, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
