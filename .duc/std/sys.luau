export type sys = {
    hostname: () -> string,
    name: () -> string
}

local sys = {}

function sys.hostname(): string
    return ""
end

function sys.name(): string
    return ""
end

return sys :: sys