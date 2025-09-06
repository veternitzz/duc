local fs = require("@self/fs")
local io = require("@self/io")
local sys = require("@self/sys")

export type std = {
    fs: fs.Fs,
    io: io.Io
}

return {
    fs = fs :: fs.Fs,
    io = io :: io.Io,
    sys = sys :: sys.sys
} :: std