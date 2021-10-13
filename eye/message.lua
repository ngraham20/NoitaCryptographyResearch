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

function Message:displayLines()
    local tui = require "tui"
    local rows = SplitBy(self.text, self.delimiter)
    tui.header(self.name..": Lines", string.len(rows[1]) + 2)
    for _,row in ipairs(rows) do
        print(row)
    end
end

function Message.weaveLinestoTrigrams(l1, l2)
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
    until iter >= (string.len(l1)+string.len(l2)//3)

    for _, value in pairs(tris) do
        io.write(value..' ')
    end
end

function Message:displayTrigrams()
    local tui = require "tui"
    tui.header(self.name..": Trigrams", 41)
    local full = self.text:gsub(self.delimiter, '')
    local allTris = {}
    local trigramCount = 0
    local iter = 1

    repeat
        table.insert(allTris, full:sub(iter,iter+1)..full:sub(iter+39,iter+39))
        table.insert(allTris, (full:sub(iter+2, iter+2)..full:sub(iter+40, iter+41)):reverse())
        trigramCount=trigramCount+2
        if trigramCount % 26 ~= 0 then
            iter=iter+3
        elseif trigramCount % 26 == 0 then
            iter=iter+42
        end
    until trigramCount >= (string.len(full)//3)

    for i, value in pairs(allTris) do
        io.write(value..' ')
        if i % 26 == 0 then
            print()
        end
    end
end

return Message