function LoadJson (file)
    local lunajson = require 'lunajson'
    local fh = io.open(file, "r")
    if not file then return nil end
    local data = fh:read "*a"
    fh:close()
    return lunajson.decode(data)
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