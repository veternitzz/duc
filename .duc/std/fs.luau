export type Fs = {
    read: (kind: "d" | "f", path: string) -> (string | { string }),
    exists: (path: string) -> (boolean),
    writeFile: (path: string, contents: string) -> (),
    writeDir: (path: string) -> ()
}

local fs = {}

function fs.read(kind: "d" | "f", path: string): string | { string }
    return "", {""}
end

function fs.exists(path: string): boolean
    return true, false
end

function fs.writeFile(path: string, contents: string): ()
    return
end

function fs.writeDir(path: string): ()
    return
end

return fs :: Fs