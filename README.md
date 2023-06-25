<div align="center">  
  <img alt="image" src="https://user-images.githubusercontent.com/87907656/174868343-f9ac7940-c49f-47fb-8f9d-a48ece0fc907.png">
  
  #### Kusa is a simple CLI tool that works on any platform and displays GitHub contribution graphs.
  
  ![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=green&style=flat-square)
  ![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)
  [![Latest Release](https://img.shields.io/github/v/release/Ryu0118/Kusa?style=flat-square)](https://github.com/Ryu0118/Kusa/releases/latest)
  ![Platform Compatibility](https://img.shields.io/badge/Platform%20Compatibility-macos%20%7C%20linux%20%7C%20windows-orange)
  [![Twitter](https://img.shields.io/twitter/follow/ryu_hu03?style=social)](https://twitter.com/ryu_hu03)
</div>

## Installation
### Homebrew (only macOS)

```sh
$ brew tap Ryu0118/Kusa
$ brew install kusa
```
or download the appropriate file for your device from [releases](https://github.com/Ryu0118/Kusa/releases/tag/0.0.2)

### Building From Source

To build and run Kusa on your machine put your GitHub Personal Access Token as the
"GITHUB_ACCESS_TOKEN" variable (src/main.rs, line 9), your token does not need any
scopes.

```sh
static GITHUB_ACCESS_TOKEN : &str = "GITHUB_ACCESS_TOKEN";
```
then run this

```sh
$ cargo run <github user name>
```

## How To Generate A Github Access Token

Go to [github developer settings](https://github.com/settings/tokens).

Set the expiration date to "No Expiration".

Click "Generate Token" and then copy the token it generates.

## Known Issues

- Terminal.app on macOS does not support 24-bit color, so colors are not displayed,
therefore, use iTerm2, Hyper, Warp or other terminals to display colors correctly.

## Usage

### `kusa --help`

```
USAGE:
    kusa [OPTIONS] <github user name>

ARGS:
    <github user name>

OPTIONS:
    -h, --help           Print help information
    -s, --hide-streak
    -V, --version        Print version information
```

### `kusa Ryu0118`

<img alt="image" width="858" src="https://github.com/Ryu0118/Kusa/assets/87907656/a935a31e-d0b8-4e63-9d49-dfbefe6c84d5">
