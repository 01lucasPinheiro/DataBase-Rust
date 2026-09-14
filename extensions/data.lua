register_extension("data_", {
    on_add = function(key, value, db_get, db_find_by_value)
        local y, m, d = string.match(value, "^(%d%d%d%d)%-(%d%d)%-(%d%d)$")
        if not y then return false, "Formato invalido. Use AAAA-MM-DD" end
        y, m, d = tonumber(y), tonumber(m), tonumber(d)
        
        if m < 1 or m > 12 then return false, "Mes inexistente" end
        
        local days = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
        local is_leap = (y % 4 == 0 and y % 100 ~= 0) or (y % 400 == 0)
        if is_leap then days[2] = 29 end
        
        if d < 1 or d > days[m] then return false, "Dia invalido para este mes/ano" end
        
        return true, value
    end,

    on_get = function(key, value)
        local y, m, d = string.match(value, "^(%d%d%d%d)%-(%d%d)%-(%d%d)$")
        return d .. "/" .. m .. "/" .. y
    end
})