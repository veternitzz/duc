--!nolint
--!nocheck

-- // Globals
declare function warn(s: string): ()
declare function print(s: string): ()

-- // Libraries
declare fs: {
    read: (kind: "f" | "d", path: string) -> string | { string },
    writeFile: (path: string, contents: string) -> (),
    writeDir: (path: string) -> ()
}

declare io: {
    write: (s: string) -> (),
    ewrite: (s: string) -> (),
    readLine: () -> string,
}
