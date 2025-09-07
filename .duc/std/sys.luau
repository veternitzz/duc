export type sys = {
    hostname: () -> string,
    name: () -> string,
    memory: () -> MemoryInfo,
    uptime: () -> number
}

local sys = {}

function sys.hostname(): string
    return ""
end

function sys.name(): string
    return ""
end

export type MemoryInfo = {
    total: number,
    free: number
}

function sys.memory(): MemoryInfo
    return {} :: MemoryInfo
end

function sys.uptime(): number
    return 0
end

return sys :: sys