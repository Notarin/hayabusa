# config.lua

The lua file is a user defined file which generates the info half of the fetch.\
The hayabusa binary injects the system information into a global object named `system_info`.

## system_info
| Field         | Data Type    | Purpose                                       |
|---------------|--------------|-----------------------------------------------|
| `distro`      | String       | Holds the name of the distribution.           |
| `cpu`         | String       | Stores the CPU name or information.           |
| `motherboard` | String       | Contains the motherboard name or information. |
| `kernel`      | String       | Provides the kernel version or information.   |
| `gpus`        | Table (List) | An indexed list of GPU names or information.  |
| `memory`      | Table        | Table containing memory usage details.        |
| `disks`       | Table (List) | A table with disk information.                |
| `local_ip`    | String       | The local IP address.                         |
| `public_ip`   | String       | The public IP address.                        |
| `hostname`    | String       | The hostname of the system.                   |
| `boot_time`   | Number       | System boot time (likely in Unix timestamp).  |
| `packages`    | Table (List) | An indexed list of package manager counts.    |

## memory
| Field   | Data Type | Purpose                               |
|---------|-----------|---------------------------------------|
| `used`  | Number    | Amount of memory currently in use.    |
| `total` | Number    | Total amount of memory in the system. |

## disks
| Field   | Data Type | Purpose                  |
|---------|-----------|--------------------------|
| `name`  | String    | Name of the disk.        |
| `used`  | Number    | Used space of the disk.  |
| `total` | Number    | Total space of the disk. |

## packages
| Field    | Data Type | Purpose                              |
|----------|-----------|--------------------------------------|
| `pacman` | Number    | Count of packages managed by Pacman. |
| `winget` | Number    | Count of packages managed by WinGet. |
| `dnf`    | Number    | Count of packages managed by DNF.    |

# config.toml

| Configuration      | Data Type           | Description                              | Options (if applicable)                    |
|--------------------|---------------------|------------------------------------------|--------------------------------------------|
| `spacing`          | Struct              | Settings for spacing within the display. |                                            |
| - `middle_padding` | u8                  | Padding between elements.                |                                            |
| - `inner_padding`  | Struct (Padding)    | Padding inside elements.                 |                                            |
| - - `top`          | u8                  | Top padding.                             |                                            |
| - - `bottom`       | u8                  | Bottom padding.                          |                                            |
| - - `left`         | u8                  | Left padding.                            |                                            |
| - - `right`        | u8                  | Right padding.                           |                                            |
| - `outer_padding`  | Struct (Padding)    | Padding outside elements.                |                                            |
| `border`           | Struct              | Settings for the border.                 |                                            |
| - `enabled`        | bool                | Toggle border on/off.                    |                                            |
| - `ansi_color`     | String              | ANSI color code for the border.          |                                            |
| - `border_chars`   | Struct              | Characters used for drawing the border.  |                                            |
| `ascii_art`        | Struct              | Settings for ASCII art display.          |                                            |
| - `size`           | Enum (AsciiSize)    | Size of ASCII art.                       | `Small`, `Big`                             |
| - `placement`      | Enum (ArtPlacement) | Placement of ASCII art.                  | `Top`, `Bottom`, `Left`, `Right`           |
| - `alignment`      | Enum (Alignment)    | Alignment of ASCII art.                  | `Left`, `Center`, `Right`, `Top`, `Bottom` |
| - `backend`        | Struct              | Backend settings for ASCII art.          |                                            |
| - - `engine`       | Enum (Engine)       | Engine used for ASCII art.               | `Ascii`, `Kitty`, `None`                   |
| - - `image_path`   | String              | Path to the image file (if used).        |                                            |
| - - `image_width`  | u16                 | Width of the image (if used).            |                                            |
| - `ascii_art_file` | String              | File path for ASCII art.                 |                                            |
