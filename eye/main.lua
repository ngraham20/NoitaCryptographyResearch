function LoadJson (file)
    local lunajson = require 'lunajson'
    local fh = io.open(file, "r")
    if not file then return nil end
    local data = fh:read "*a"
    fh:close()
    return lunajson.decode(data)
end

function DisplayMessageTrigram(eyes)
    local delimiter = eyes["delimiter"]
    local messages = eyes["messages"]
    for key, value in pairs(messages) do
        local rows = SplitBy(value, delimiter)
        print(key .. ":")
        for _,row in ipairs(rows) do
            print(row)
        end
        print()
    end
end

function DisplayMessagesLines(eyes)
    local delimiter = eyes["delimiter"]
    local messages = eyes["messages"]
    for key, value in pairs(messages) do
        DisplayMessageLines(value, delimiter)
    end
end

function DisplayMessageTrigrams(message, delimiter)

end

function DisplayMessageLines(message, delimiter)
    local rows = SplitBy(message, delimiter)
    for _,row in ipairs(rows) do
        print(row)
    end
end

local function displayMessagePixels(eyes)
end

function DisplayEyes(eyes, format)
    local action = {
        ["trigram"] = DisplayMessageTrigram,
        ["lines"] = DisplayMessagesLines,
        ["pixels"] = displayMessagePixels
    }
    action[format](eyes)
end

function CountEyes(eyes)
    local delimiter = eyes["deliimiter"]
end

function SplitBy (inputstr, sep)
    if sep == nil then
        sep = "%s"
    end
    local t={}
    for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
        table.insert(t, str)
    end
    return t
end

local eyes = LoadJson("eyes.json")
local Message = require "message"
local m = Message:fromEyes(eyes, "east1")
m:displayLines()
m:displayTrigrams()

-- incrememnt line 1 by 2, then line 2 by 1
-- when line 1 is done, jump to line 3

-- 1, 3, 5, and 2, 4, 6