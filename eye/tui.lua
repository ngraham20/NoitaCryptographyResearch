TUI = {}

function TUI.header(header, size)
    local headersize = string.len(header)
    local hprespace = (size-headersize) // 2
    print(string.rep("─", size))
    print(string.rep(" ", hprespace) .. header)
    print(string.rep("─", size))
end

return TUI