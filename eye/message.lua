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

function Message:displayTrigrams()
    local tui = require "tui"
    tui.header(self.name..": Trigrams", 41)
    local full = self.text:gsub(self.delimiter, '')
    local allTris = {}
    local trigramCount = 0
    local iter = 1
    -- 26 trigrams per row

    -- TODO : This breaks because it "skips ahead" the expected line length.
    -- The last few lines are too short, so it skips too far.
    -- Probably, it will be easier to skip to the next desired delimiter instead of removing it

    -- maybe make a "weave" function that takes two lines and weaves them together in trigrams
    -- keep in account that it could end on an odd trigram
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