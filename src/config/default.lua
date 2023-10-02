-- ===============================
-- Hayabusa User Configuration File
-- ===============================

-- [ANSI Color Codes]
-- ===================
-- These control text color in the terminal.
local ansi_green = "\27[32m"
local ansi_blue = "\27[34m"
local ansi_reset = "\27[0m"

-- [Helper Functions]
-- ==================
-- Additional functions for converting and formatting data.

-- Convert bytes to GiB
function bytes_to_gib(bytes)
    return string.format("%.2f", bytes / 1024 / 1024 / 1024)
end

-- Get GPU Information
function gpuInfo()
    local gpu_str = ""
    for i, gpu in ipairs(system_info.gpus) do
        gpu_str = gpu_str .. "  " .. ansi_green .. "GPU" .. " ❯ " .. ansi_reset .. gpu .. "\n"
    end
    return gpu_str
end

-- Get Disk Information
function diskInfo()
    local disk_str = ""
    for i, disk in ipairs(system_info.disks) do
        disk_str = disk_str .. "  " .. ansi_green .. "Disk:" .. disk.name .. ":" .. " ❯ " .. ansi_reset .. bytes_to_gib(disk.used) .. "GiB / " .. bytes_to_gib(disk.total) .. "GiB\n"
    end
    return disk_str
end

-- [Main Configuration]
-- ====================
-- Edit the lines below to customize the output.

-- line format
local format = {
    distro = "  " .. ansi_green .. "Distro ❯ " .. ansi_reset .. system_info.distro .. "\n",
}

-- System Info
local config = {
    header = ansi_blue .. "╒═══════════════════════════════════════════╕\n" .. ansi_reset,
    distro =              "  " .. ansi_green .. "Distro ❯ " .. ansi_reset .. system_info.distro .. "\n",
    cpu =                 "  " .. ansi_green .. "CPU ❯ " .. ansi_reset .. system_info.cpu .. "\n",
    motherboard =         "  " .. ansi_green .. "Motherboard ❯ " .. ansi_reset .. system_info.motherboard .. "\n",
    kernel =              "  " .. ansi_green .. "Kernel ❯ " .. ansi_reset .. system_info.kernel .. "\n",
    memory =              "  " .. ansi_green .. "Memory ❯ " .. ansi_reset .. bytes_to_gib(system_info.memory.used) .. "GiB / " .. bytes_to_gib(system_info.memory.total) .. "GiB\n",
    local_ip =            "  " .. ansi_green .. "Local IP ❯ " .. ansi_reset .. system_info.local_ip .. "\n",
    footer = ansi_blue .. "╘═══════════════════════════════════════════╛" .. ansi_reset,
}

-- [Generate Output]
-- =================

result = config.header ..
         config.distro ..
         config.cpu ..
         config.motherboard ..
         config.kernel ..
         gpuInfo() ..
         config.memory ..
         diskInfo() ..
         config.local_ip ..
         config.footer
