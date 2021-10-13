local trigram = {}

function trigram:new(substring)
    return {substring[0], substring[1], substring[2]}
end

function trigram:toString()
end

function trigram:toImage()
end

return trigram