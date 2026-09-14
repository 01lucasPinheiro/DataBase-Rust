register_extension("aluno_", {
    on_add = function(key, value, db_get, db_find_by_value)
        local nome, idade_str, resp_key = string.match(value, "^([^,]+),(%d+),(.+)$")
        if not nome then
            return false, "Formato invalido. Use: Nome,Idade,chave_responsavel"
        end
        
        local idade = tonumber(idade_str)
        if idade ~= 7 and idade ~= 8 then
            return false, "Idade incompativel. Alunos do 2o Ano devem ter 7 ou 8 anos."
        end
        
        local resp_existe = db_get(resp_key)
        if not resp_existe then
            return false, "O responsavel referenciado na chave '" .. resp_key .. "' nao existe no banco. Cadastre o CPF primeiro."
        end
        
        return true, value
    end,

    on_get = function(key, value)
        local nome, idade, resp = string.match(value, "^([^,]+),(%d+),(.+)$")
        return "Aluno(a): " .. nome .. " | Turma: 2o Ano Fundamental | Reponsavel cadastrado em: " .. resp
    end
})