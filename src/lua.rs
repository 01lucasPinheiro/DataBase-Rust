use mlua::prelude::*;
use std::fs;
use crate::storage::CacheDatabase;

pub struct LuaEngine {
    lua: Lua,
}

impl LuaEngine {
    pub fn new() -> LuaResult<Self> {
        let lua = Lua::new();
        
        let boot_script = r#"
            EXTENSIONS = {}
            function register_extension(prefix, callbacks)
                EXTENSIONS[prefix] = callbacks
            end

            function find_extension(key)
                for prefix, callbacks in pairs(EXTENSIONS) do
                    if string.sub(key, 1, string.len(prefix)) == prefix then
                        return callbacks
                    end
                end
                return nil
            end

            function dispatch_add(key, value)
                local ext = find_extension(key)
                if ext and ext.on_add then
                    return ext.on_add(key, value, db_get, db_find_by_value)
                end
                return true, value
            end

            function dispatch_get(key, value)
                local ext = find_extension(key)
                if ext and ext.on_get then
                    return ext.on_get(key, value)
                end
                return value
            end
        "#;
        lua.load(boot_script).exec()?;

        if let Ok(entries) = fs::read_dir("extensions") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                    if let Ok(code) = fs::read_to_string(&path) {
                        if let Err(e) = lua.load(&code).exec() {
                            eprintln!("ERRO ao carregar extensão {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(LuaEngine { lua })
    }

    pub fn process_add(&self, db: &CacheDatabase, key: &str, value: &str) -> Result<String, String> {
        let result: mlua::Result<(bool, Option<String>)> = self.lua.scope(|scope| {
            let globals = self.lua.globals();
            
            let get_fn = scope.create_function(|_, k: String| Ok(db.get(&k)))?;
            globals.set("db_get", get_fn)?;

            let find_fn = scope.create_function(|_, val: String| Ok(db.find_by_value(&val)))?;
            globals.set("db_find_by_value", find_fn)?;

            let dispatch: mlua::Function = globals.get("dispatch_add")?;
            dispatch.call((key, value))
        });

        match result {
            Ok((true, Some(transformed))) => Ok(transformed),
            Ok((true, None)) => Ok(value.to_string()),
            Ok((false, Some(err))) => Err(err),
            Ok((false, None)) => Err("Erro de validação desconhecido na extensão.".to_string()),
            Err(e) => Err(format!("Falha no script Lua: {}", e)),
        }
    }

    pub fn process_get(&self, key: &str, value: &str) -> Result<String, String> {
        let result: mlua::Result<String> = self.lua.scope(|scope| {
            let globals = self.lua.globals();
            let dispatch: mlua::Function = globals.get("dispatch_get")?;
            dispatch.call((key, value))
        });

        match result {
            Ok(formatted) => Ok(formatted),
            Err(e) => Err(format!("Falha no script Lua: {}", e)),
        }
    }
}   