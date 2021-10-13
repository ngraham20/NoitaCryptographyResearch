require "util"

local eyes = LoadJson("eyes.json")
local Message = require "message"
local m = Message:fromEyes(eyes, "east1")
m:display("lines")