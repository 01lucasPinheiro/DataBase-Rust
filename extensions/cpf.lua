register_extension("cpf_", {
    on_add = function(key, value, db_get, db_find_by_value)
        if string.len(value) ~= 11 or not tonumber(value) then
            return false, "O CPF deve ter 11 numeros, sem formatacao"
        end

        local primeiro = string.sub(value, 1, 1)
        if value == string.rep(primeiro, 11) then
            return false, "CPF invalido (numeros repetidos)"
        end

        local soma, resto = 0, 0
        for i = 1, 9 do soma = soma + tonumber(string.sub(value, i, i)) * (11 - i) end
        resto = (soma * 10) % 11
        if resto == 10 or resto == 11 then resto = 0 end
        if resto ~= tonumber(string.sub(value, 10, 10)) then return false, "Digito verificador 1 invalido" end

        soma = 0
        for i = 1, 10 do soma = soma + tonumber(string.sub(value, i, i)) * (12 - i) end
        resto = (soma * 10) % 11
        if resto == 10 or resto == 11 then resto = 0 end
        if resto ~= tonumber(string.sub(value, 11, 11)) then return false, "Digito verificador 2 invalido" end

        -- Unicidade via banco
        local owner_key = db_find_by_value(value)
        if owner_key and owner_key ~= key then
            return false, "CPF ja cadastrado na chave: " .. owner_key
        end

        return true, value
    end,

    on_get = function(key, value)
        return string.sub(value, 1, 3) .. "." .. string.sub(value, 4, 6) .. "." .. string.sub(value, 7, 9) .. "-" .. string.sub(value, 10, 11)
    end
})