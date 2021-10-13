require "util"
require "message"

local eyes = LoadJson("eyes.json")
local m = Message:fromEyes(eyes, arg[1])
for i=2,#arg do
    m:display(arg[i])
end