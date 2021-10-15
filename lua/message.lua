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

function Message:display(format)
    local action = {
        ["trigrams"] = DisplayTrigrams,
        ["lines"] = DisplayLines,
        ["pixels"] = DisplayPixels
    }
    action[format](self)
end

function DisplayPixels(self)
end

function DisplayLines(self)
    local tui = require "tui"
    local rows = SplitBy(self.text, self.delimiter)
    tui.header(self.name..": Lines", string.len(rows[1]) + 2)
    for _,row in ipairs(rows) do
        print(row)
    end
end

local function weaveLinestoTrigrams(l1, l2)
    local tris = {}
    local iter = 1

    repeat
        local tri = l1:sub(iter+(iter//2),iter+((iter+1)//2))..l2:sub(iter+((iter-1)//2), iter+(iter//2))

        if iter % 2 == 0 then
            table.insert(tris, string.reverse(tri))
        else
            table.insert(tris, tri)
        end
        iter = iter + 1
    until iter > ((string.len(l1)+string.len(l2))//3)
    return tris
end

function DisplayTrigrams(self)
    local tui = require "tui"
    local rows = SplitBy(self.text, self.delimiter)
    tui.header(self.name..": Trigrams", string.len(rows[1]) + 2)
    local tris = {}

    -- get two rows at a time. We can assume pairs of rows.
    for i=1,#rows,2 do
        local wovenTris = weaveLinestoTrigrams(rows[i], rows[i+1])
        for _,v in ipairs(wovenTris) do 
            table.insert(tris, v)
        end
    end

    for i,value in pairs(tris) do
        io.write(value..' ')
        if i % 26 == 0 then
            print()
        end
    end
    print()
end

return Message