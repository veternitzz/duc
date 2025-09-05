local fs = require("@self/fs")
local io = require("@self/io")

export type std = {
    fs: fs.Fs,
    io: io.Io
}

return {
    fs = fs :: fs.Fs,
    io = io :: io.Io
} :: std