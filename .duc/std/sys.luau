export type sys = {
    hostname: () -> string,
    name: () -> string,
    memory: () -> MemoryInfo
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

return sys :: sys