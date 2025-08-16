# Hayabusa
|                      |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Languages            | ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Lua](https://img.shields.io/badge/lua-%232C2D72.svg?style=for-the-badge&logo=lua&logoColor=white)                                                                                                                                                                                                                                                                                                                               |
| Packaged Ascii Logos | ![Arch](https://img.shields.io/badge/Arch%20Linux-1793D1?logo=arch-linux&logoColor=fff&style=for-the-badge) ![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white) ![Ubuntu](https://img.shields.io/badge/Ubuntu-E95420?style=for-the-badge&logo=ubuntu&logoColor=white)<br/>![Gentoo](https://img.shields.io/badge/Gentoo-54487A?style=for-the-badge&logo=gentoo&logoColor=white) ![NixOS](https://img.shields.io/badge/NixOS-5277c3?style=for-the-badge&logo=nixos&logoColor=FFFFFF) |

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
This program is packaged firstly as a nix package available through the flakes
outputs. Furthermore there is a nixosModule provided, simply add
`hayabusa.nixosModules.default` to your modules, then set
`services.hayabusa.enable` to true. Ensure you add the package provided by the
flake to your `PATH`.
If you do not use NixOS, I have decided to not deal with packaging it myself.
There are an unreal amount of package managers, and frankly literally anyone can
package it for a package manager and make it avilable to everyone.
If you really want hayabusa, and there exists no package for you, the binary and
the service file are in releases. Feel free to deploy it yourself.

If you *really* can't do that, and still want to use hayabusa, the below
solution exists at your own risk. I recommend against using it, as every
operating system is wildly different, and there is no guarantee it will not
break something.

- Download `install-linux.sh` from releases.
- Run it in the terminal.

Finished! You can now run `hayabusa` to run the program.

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
