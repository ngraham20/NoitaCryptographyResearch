require "util"
local Message = require "message"

local eyes = LoadJson("eyes.json")
local m = Message:fromEyes(eyes, "east1")
m:display("trigrams")