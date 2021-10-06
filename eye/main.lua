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

function DisplayMessageLines(eyes)
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

local function displayMessagePixels(eyes)
end

function DisplayEyes(eyes, format)
    local action = {
        ["trigram"] = DisplayMessageTrigram,
        ["lines"] = DisplayMessageLines,
        ["pixels"] = displayMessagePixels
    }
    action[format](eyes)
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
DisplayEyes(eyes, "lines")