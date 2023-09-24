function bytes_to_gib(bytes)
    return string.format("%.2f", bytes / 1024 / 1024 / 1024)
end

local ansi_green = "\27[32m"
local ansi_blue = "\27[34m"
local ansi_reset = "\27[0m"

local gpu_str = ""
for i, gpu in ipairs(gpus) do
    gpu_str = gpu_str .. "  " .. ansi_green .. "GPU" .. " ❯ " .. ansi_reset .. gpu .. "\n"
end

local disk_str = ""
for i, disk in ipairs(disks) do
    disk_str = disk_str .. "  " .. ansi_green .. "Disk:" .. disk.name .. ":" .. " ❯ " .. ansi_reset .. bytes_to_gib(disk.used) .. "GiB / " .. bytes_to_gib(disk.total) .. "GiB\n"
end

result = ansi_blue .. "╒═══════════════════════════════════════════╕\n" .. ansi_reset ..
         "  " .. ansi_green .. "Distro ❯ " .. ansi_reset .. distro .. "\n" ..
         "  " .. ansi_green .. "CPU ❯ " .. ansi_reset .. cpu .. "\n" ..
         "  " .. ansi_green .. "Motherboard ❯ " .. ansi_reset .. motherboard .. "\n" ..
         "  " .. ansi_green .. "Kernel ❯ " .. ansi_reset .. kernel .. "\n" ..
         gpu_str ..
         "  " .. ansi_green .. "Memory ❯ " .. ansi_reset .. bytes_to_gib(memory.used) .. "GiB / " .. bytes_to_gib(memory.total) .. "GiB\n" ..
         disk_str ..
         "  " .. ansi_green .. "Local IP ❯ " .. ansi_reset .. local_ip .. "\n" ..
         ansi_blue .. "╘═══════════════════════════════════════════╛" .. ansi_reset
