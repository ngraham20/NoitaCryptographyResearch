Message = {
    name = "",
    text = "",
    delimiter = -1
}

function Message:fromEyes(eyes, name)
    local m = {}
    setmetatable(m, self)
    self.__index = self
    self.name = name
    self.text = eyes["messages"][name]
    self.delimiter = eyes["delimiter"]
    return m
end

function Message:printName()
    print(self["name"])
end

function Message:displayLines()
    -- local rows = SplitBy(self["text"], self["delimiter"])
    -- print(self["name"])
    -- for _,row in ipairs(rows) do
    --     print(row)
    -- end
    print(self["text"])
end

return Message