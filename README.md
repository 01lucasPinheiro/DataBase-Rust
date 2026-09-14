# Banco de Dados em Memória: Rust + Lua

**Integrantes do Grupo:**
- Lucas Pinheiro

## 1. Como compilar e executar

O projeto requer a linguagem Rust instalada. Para rodar a partir do código fonte:

1. Navegue até a raiz do projeto (onde está o `Cargo.toml`).
2. Compile e execute o projeto com o comando:
   ```bash
   cargo run
   ```
3. O terminal interativo iniciará com o prompt `> `. Você pode começar a digitar os comandos `ADD`, `GET` e `EXIT`.
4. Para rodar um roteiro de testes em lote via pipe, utilize:
   ```bash
   cat casos_teste.txt | cargo run
   ```

## 2. O Protocolo de Registro de Extensões

O banco de dados descobre as extensões em tempo de execução varrendo o diretório `extensions/`. O protocolo de registro funciona injetando um script de inicialização (boot) na VM do Lua.

Dentro do Rust (`lua.rs`), a VM expõe uma função global chamada `register_extension`. Cada script `.lua` na pasta `extensions/` deve chamar essa função informando dois argumentos:
1. O **prefixo da chave** que a extensão trata (ex: `"cpf_"`).
2. Uma **tabela com callbacks** para as operações: `on_add` e/ou `on_get`.

A tabela `EXTENSIONS` mapeia cada prefixo às suas respectivas funções. Quando o usuário executa um comando, o Rust aciona funções despachantes globais (`dispatch_add` ou `dispatch_get`) que procuram na tabela de prefixos e, se encontrarem correspondência, delegam a execução para o script.

### Estruturas de Retorno de Sucesso e Erro

A transição do erro no Lua até a mensagem impressa na tela ocorre através das funções de despacho que convertem a saída do script em tipos nativos do Rust.

No `ADD`, a extensão deve retornar duas variáveis no Lua: um booleano (sucesso/falha) e uma string (que será o valor transformado em caso de sucesso, ou a mensagem explicando o erro em caso de falha).
- O Rust captura esse retorno duplo em uma estrutura `Result<(bool, Option<String>)>`.
- Se o booleano for `false`, o Rust extrai a mensagem, acopla o prefixo, monta a string final `ERRO: {mensagem}` e a imprime na tela através do módulo `repl.rs`.
- Se for `true`, o Rust sabe que a validação passou e salva a `String` final no banco de dados em memória.

## 3. Como acrescentar uma extensão nova

Para criar uma nova extensão, qualquer pessoa pode seguir estes passos, sem a necessidade de reescrever ou recompilar o código Rust:

1. Crie um novo arquivo `.lua` dentro do diretório `extensions/` (exemplo: `cep.lua`).
2. No arquivo, chame a função `register_extension("seu_prefixo_", { ... })`.
3. Defina a função `on_add = function(key, value, db_get, db_find_by_value)`. Ela deve retornar `true, "valor_formatado"` se a entrada for válida, ou `false, "Mensagem de erro"` se for inválida.
4. Defina a função `on_get = function(key, value)`. Ela deve retornar o valor formatado para a visualização na tela.
5. Reinicie o banco de dados. A extensão será localizada pela varredura da pasta e carregada automaticamente.

## 4. Consulta ao banco na validação (Resolução de Concorrência)

Durante a validação, algumas extensões precisam ler o banco de dados (ex: verificar unicidade de CPF ou existência de outra chave). Contudo, a operação do comando `ADD` já está prestes a escrever no banco de dados.

**A Solução:** Resolvemos o possível problema de impasses (deadlocks) separando temporalmente a fase de validação e a fase de gravação. 
1. O módulo `repl.rs` aciona a VM do Lua chamando `process_add()`.
2. Nesse momento, passamos referências locais do banco para funções de leitura que são injetadas temporariamente no escopo do Lua (`db_get` e `db_find_by_value`). O Lua faz a validação consultando o banco. A trava da estrutura `RwLock` é fechada estritamente para *leitura* (read lock), garantindo que nada fique travado indefinidamente.
3. Só **depois** que o Lua retorna um sucesso na validação em `process_add()`, o fluxo em `repl.rs` avança e chama `db.set()`. Nesse instante sim o Rust obtém uma trava de *escrita* (write lock) no banco. Como a validação pelo script já liberou as travas de leitura, a escrita flui sem bloqueios indesejados, garantindo que a consulta no Lua reflita o estado exato e instantâneo do armazenamento.

## 5. Extensão Proposta: Validação de Alunos (`aluno.lua`)

A extensão de livre escolha criada pelo grupo é a `aluno_`, que cuida do cadastro de estudantes. A entrada exige o formato exato `Nome,Idade,chave_responsavel`.

**Motivação e Inovação:**
A ideia desta extensão reflete o contexto pedagógico real de elaboração de rotinas e planos de aula, já que a validação impõe a regra estrita de que a criança tenha exclusivamente 7 ou 8 anos — requisito alinhado diretamente com as diretrizes e cronogramas de turmas de segundo ano do ensino fundamental.

**O que ela exercita de novo?**
Diferente da extensão obrigatória do CPF (que consulta o banco para procurar se o próprio valor de CPF já está duplicado em algum canto) e diferente da Data (que roda cálculos sem consultar o banco), a extensão `aluno_` faz um exercício de **integridade referencial (Relacionamento de dependência chave-chave)**.
Antes de autorizar o cadastro de um aluno novo, ela aciona o `db_get` para investigar se a "chave do responsável" referenciada (que pode ser um CPF) de fato já existe no sistema. Isso demonstra que o motor desenhado é genérico o suficiente para suportar regras de negócio que interligam chaves e fluxos cruzados na base de dados.

## 6. Módulos do Projeto e Dependências

As quatro áreas centrais foram isoladas, conforme solicitado:
- **`main.rs`**: O ponto de entrada. Apenas coordena a inicialização e aciona o loop.
- **`storage.rs`**: O armazenamento. Gerencia a `HashMap` em memória com controle de concorrência (`RwLock`). Não sabe da existência de Lua ou comandos.
- **`parser.rs`**: Interpretação. É responsável puramente por transformar o texto do usuário na estrutura `Command` em Rust.
- **`repl.rs`**: A Leitura de entrada. Controla o laço de exibição do prompt `> `, orquestrando a lógica de fluxo entre o parser, o executor Lua e o armazenamento. 
- **`lua.rs`**: A Ponte. É o único módulo do projeto que depende do crate `mlua`. Ele isola todas as operações que lidam com extensões, impedindo que o resto do código precise saber como a validação é disparada.

## 7. Decisões de Projeto Adicionais

- **Uso do `mlua::scope`:** Optou-se por usar a função de "scope" temporário (closure) da biblioteca `mlua` para criar as funções `db_get` e `db_find_by_value`. Essa decisão evitou que o `CacheDatabase` inteiro precisasse implementar os traits de `UserData` do Lua, deixando o gerenciamento da memória (lifetimes) inteiramente do lado seguro do Rust.
- **Comportamento da Interface:** No `parser.rs`, implementamos lógicas de `.trim()` e tratamentos para descartar linhas perfeitamente vazias em vez de derrubar o prompt ou emitir erros confusos. Isso torna a interface muito mais próxima do que se espera em conectores como o do Redis.
