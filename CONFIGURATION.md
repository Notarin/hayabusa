# config.lua

The lua file is a user defined file which generates the fetch part of the program.\
The hayabusa binary injects the system information into a global object named `system_info`.

## system_info
| Field         | Data Type    | Purpose                                        |
|---------------|--------------|------------------------------------------------|
| `distro`      | String       | Holds the name of the distribution.            |
| `cpu`         | String       | Stores the CPU name.                           |
| `motherboard` | String       | Contains the motherboard name.                 |
| `kernel`      | String       | Provides the kernel version.                   |
| `gpus`        | Table (List) | An indexed list of GPU names.                  |
| `memory`      | Table        | Table containing memory usage details.         |
| `disks`       | Table (List) | A table with disk information.                 |
| `local_ip`    | String       | The local IP address.                          |
| `public_ip`   | String       | The public IP address.                         |
| `hostname`    | String       | The hostname of the system.                    |
| `boot_time`   | Number       | System boot time (usually for finding uptime). |
| `packages`    | Table (List) | An indexed list of package manager counts.     |

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

| Configuration      | Data Type    | Description                                      | Options (if applicable) or Comments        |
|--------------------|--------------|--------------------------------------------------|--------------------------------------------|
| `spacing`          | Struct       | Settings for spacing within the display.         |                                            |
| - `middle_padding` | u8 (Number)  | Padding between the art and the fetch.           |                                            |
| - `inner_padding`  | Struct       | Padding inside the border.                       |                                            |
| - - `top`          | u8 (Number)  | Inner top padding.                               |                                            |
| - - `bottom`       | u8 (Number)  | Inner bottom padding.                            |                                            |
| - - `left`         | u8 (Number)  | Inner left padding.                              |                                            |
| - - `right`        | u8 (Number)  | Inner right padding.                             |                                            |
| - `outer_padding`  | Struct       | Padding outside the border.                      |                                            |
| - - `top`          | u8 (Number)  | Outer top padding.                               |                                            |
| - - `bottom`       | u8 (Number)  | Outer bottom padding.                            |                                            |
| - - `left`         | u8 (Number)  | Outer left padding.                              |                                            |
| - - `right`        | u8 (Number)  | Outer right padding.                             |                                            |
| `border`           | Struct       | Settings for the border.                         |                                            |
| - `enabled`        | bool         | Toggle border on/off.                            |                                            |
| - `ansi_color`     | String       | ANSI color code for the border.                  |                                            |
| - `border_chars`   | Struct       | Characters used for drawing the border.          |                                            |
| - - `top_left`     | char         | Character for top-left corner of the border.     |                                            |
| - - `top_right`    | char         | Character for top-right corner of the border.    |                                            |
| - - `bottom_left`  | char         | Character for bottom-left corner of the border.  |                                            |
| - - `bottom_right` | char         | Character for bottom-right corner of the border. |                                            |
| - - `horizontal`   | char         | Character for horizontal lines of the border.    |                                            |
| - - `vertical`     | char         | Character for vertical lines of the border.      |                                            |
| `ascii_art`        | Struct       | Settings for art display.                        |                                            |
| - `size`           | Enum         | Size of the ASCII art.                           | `Small`, `Big`                             |
| - `placement`      | Enum         | Placement of art.                                | `Top`, `Bottom`, `Left`, `Right`           |
| - `alignment`      | Enum         | Alignment of art.                                | `Left`, `Center`, `Right`, `Top`, `Bottom` |
| - `backend`        | Struct       | Backend settings for the art.                    |                                            |
| - - `engine`       | Enum         | Engine used for the art.                         | `Ascii`, `Kitty`, `None`                   |
| - - `image_path`   | String       | Path to the image file (if used).                | Program will Panic is not a valid path     |
| - - `image_width`  | u16 (Number) | Width of the image (if used).                    | In character cells                         |
| - `ascii_art_file` | String       | File path for ASCII art.                         | Leave empty("") for default                |
