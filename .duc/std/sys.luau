export type sys = {
    hostname: () -> string
}

local sys = {}

function sys.hostname(): string
    return ""
end

return sys :: sys