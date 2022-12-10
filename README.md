

<div align="center">  
  <img alt="image" src="https://user-images.githubusercontent.com/87907656/174868343-f9ac7940-c49f-47fb-8f9d-a48ece0fc907.png">
  
  #### Kusa is a cli tool that displays the Github Contributions Graph.
  
  ![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=green&style=flat-square)
  ![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)
  [![Latest Release](https://img.shields.io/github/v/release/Ryu0118/Kusa?style=flat-square)](https://github.com/Ryu0118/Kusa/releases/latest)
  ![Platform Compatibility](https://img.shields.io/badge/Platform%20Compatibility-macos%20%7C%20linux%20%7C%20windows-orange)
  [![Twitter](https://img.shields.io/twitter/follow/ryu_hu03?style=social)](https://twitter.com/ryu_hu03)
</div>

## Installation
### Homebrew (only macOS)

```
$ brew tap Ryu0118/Kusa
$ brew install kusa
```
or download the appropriate file for your device from [releases](https://github.com/Ryu0118/Kusa/releases/tag/0.0.2)

### Building From Source

To build and run Kusa in your own environment, 
Assign your Github Personal Access Token with "read:user" in environment variable as
```Bash
GITHUB_TOKEN="<your_github_token>"
```
then run this

```Rust
$ cargo run <github_user_name>
```
or run directly with
```
$ GITHUB_TOKEN=<your_github_token> cargo run <github_user_name>
```

## How To Generate A Github Access Token
Go to [github developer settings](https://github.com/settings/tokens).

Set the expiration date to "No Expiration".

No scopes are required for this application.

Click "Generate Token" and then copy the token it generates.

## Known Issue
Terminal.app on macOS does not support 24-bit color, so colors are not displayed.
Therefore, use iTerm2, Hyper, Warp or other terminals to display colors correctly.

## Usage

### `kusa --help`
```
USAGE:
    kusa <github_user_name>

ARGS:
    <github_user_name>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

### `kusa Ryu0118`
<img alt="image" src="https://user-images.githubusercontent.com/87907656/175245140-e01b8848-c5e7-4cdc-acf9-90aa2da703ed.png">
