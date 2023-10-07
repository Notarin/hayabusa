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

-- Get Username
function getUsername()
    local username = os.getenv("USER")
    if username == nil then
        username = os.getenv("USERNAME")
    end
    if username == nil then
        username = "Unknown"
    end
    return username
end

-- Get Shell
function getShell()
    local shell = os.getenv("SHELL")
    if shell == nil then
        shell = os.getenv("STARSHIP_SHELL")
    end
    if shell == nil then
        shell = "Unknown"
    end
    return shell
end

-- Get Terminal
function getTerminal()
    local terminal = os.getenv("TERM")
    if terminal == nil then
        terminal = "Unknown"
    end
    return terminal
end

-- Get Uptime
function getUptime()
    local uptime = os.time() - system_info.boot_time
    local days = math.floor(uptime / 86400)
    local hours = math.floor(uptime / 3600) % 24
    local minutes = math.floor(uptime / 60) % 60
    local seconds = uptime % 60
    return string.format("%dd %dh %dm %ds", days, hours, minutes, seconds)
end

-- Get Desktop Environment
function getDE()
    local de = os.getenv("DESKTOP_SESSION")
    if de == nil then
        de = "Unknown"
    end
    return de
end

function getPackages()
    local packages = "  " .. ansi_green .. "Package" .. " ❯ " .. ansi_reset
    for package_manager, package in pairs(system_info.packages) do
        if package > 0 then
            packages = packages .. package_manager .. ": " .. package .. " "
        end
    end
    packages = packages .. "\n"
    return packages
end

-- [Main Configuration]
-- ====================
-- Edit the lines below to customize the output.

-- line format
local format = {
    username = "  " .. ansi_green .. "Username ❯ " .. ansi_reset .. getUsername() .. "\n",
    hostname = "  " .. ansi_green .. "Hostname ❯ " .. ansi_reset .. system_info.hostname .. "\n",
    distro = "  " .. ansi_green .. "Distro ❯ " .. ansi_reset .. system_info.distro .. "\n",
    cpu = "  " .. ansi_green .. "CPU ❯ " .. ansi_reset .. system_info.cpu .. "\n",
    motherboard = "  " .. ansi_green .. "Motherboard ❯ " .. ansi_reset .. system_info.motherboard .. "\n",
    kernel = "  " .. ansi_green .. "Kernel ❯ " .. ansi_reset .. system_info.kernel .. "\n",
    gpu = gpuInfo(),
    memory = "  " .. ansi_green .. "Memory ❯ " .. ansi_reset .. bytes_to_gib(system_info.memory.used) .. "GiB / " .. bytes_to_gib(system_info.memory.total) .. "GiB\n",
    disk = diskInfo(),
    local_ip = "  " .. ansi_green .. "Local IP ❯ " .. ansi_reset .. system_info.local_ip .. "\n",
    public_ip = "  " .. ansi_green .. "Public IP ❯ " .. ansi_reset .. system_info.public_ip .. "\n",
    uptime = "  " .. ansi_green .. "Uptime ❯ " .. ansi_reset .. getUptime() .. "\n",
    shell = "  " .. ansi_green .. "Shell ❯ " .. ansi_reset .. getShell() .. "\n",
    desktop_environment = "  " .. ansi_green .. "DE ❯ " .. ansi_reset .. getDE() .. "\n",
    terminal = "  " .. ansi_green .. "Terminal ❯ " .. ansi_reset .. getTerminal() .. "\n",
    packages = getPackages(),
}

-- System Info
local config = {
    header = ansi_blue .. "╒═══════════════════════════════════════════╕\n" .. ansi_reset,
    username =            format.username,
    hostname =            format.hostname,
    distro =              format.distro,
    cpu =                 format.cpu,
    motherboard =         format.motherboard,
    kernel =              format.kernel,
    gpu =                 format.gpu,
    memory =              format.memory,
    disk =                format.disk,
    local_ip =            format.local_ip,
    public_ip =           format.public_ip,
    uptime =              format.uptime,
    shell =               format.shell,
    desktop_environment = format.desktop_environment,
    terminal =            format.terminal,
    packages =            format.packages,
    footer = ansi_blue .. "╘═══════════════════════════════════════════╛" .. ansi_reset,
}

-- [Generate Output]
-- =================

result = config.header ..
         config.username ..
         config.hostname ..
         config.distro ..
         config.cpu ..
         config.motherboard ..
         config.kernel ..
         config.gpu ..
         config.memory ..
         config.disk ..
         config.local_ip ..
         config.uptime ..
         config.shell ..
         config.desktop_environment ..
         config.terminal ..
         config.packages ..
         config.footer
