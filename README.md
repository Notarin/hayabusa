# Hayabusa
|                      |                                                                                                                                                                                                                                                                                                                            |
|----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Languages            | ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Lua](https://img.shields.io/badge/lua-%232C2D72.svg?style=for-the-badge&logo=lua&logoColor=white)                                                                                                                 |
| Packaged Ascii Logos | ![Arch](https://img.shields.io/badge/Arch%20Linux-1793D1?logo=arch-linux&logoColor=fff&style=for-the-badge) ![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white) ![Ubuntu](https://img.shields.io/badge/Ubuntu-E95420?style=for-the-badge&logo=ubuntu&logoColor=white)<br/>![Gentoo](https://img.shields.io/badge/Gentoo-54487A?style=for-the-badge&logo=gentoo&logoColor=white) |

![image](https://github.com/Notarin/hayabusa/assets/25104390/7bca823a-f64e-45af-a901-b8996bf44488)


Hayabusa is a swift rust fetch program.

When configured for speed it boasts an internal runtime of under 2ms. Despite
its speed, this is a fully featured fetch, unlike many of its brethren. It is
also extremely configurable, enabling near infinite customization.
The level of customization is from the fact that the config file is a lua
script in the users config directory.
The program is split into a user facing binary and a system service daemon. The
daemon is responsible for gathering the system information ahead of time and
having it prepared for the user facing binary. This binary then injects said
system information into the lua script and extracts the result.
While the ascii art is embedded in the binary, configuration allows for
specifying a path to a file containing the ascii art. Not only this, but
hayabusa has image support, it can display any png file as long as the
terminal supports it.

## Installation
### From source:
- Clone the repository: `git clone https://github.com/Notarin/hayabusa && cd hayabusa`
- Inside the repository, run `make build` to build the binary
- Run `sudo make install` to install

Finished! You can now run `hayabusa` to run the program.

## Uninstallation
### From source:
- Inside the repository, run `sudo make uninstall`

Finished! The program is now uninstalled.

## Usage
You will need to have the daemon running in the background before making a
request with the user facing binary. It is recommended to use a systemd
service to manage the daemon.
```
Usage: hayabusa [OPTIONS]

Options:
  -d, --daemon     Run as daemon
  -b, --benchmark  On exit print the execution time, for benchmarking
  -h, --help       Print help
```

## Configuration
Check out [CONFIGURATION.md](https://github.com/Notarin/hayabusa/blob/main/CONFIGURATION.md)!
