mod storage;
mod parser;
mod repl;
mod lua; // NOVO

fn main() {
    let data_base = storage::CacheDatabase::new();
    let lua_engine = lua::LuaEngine::new().expect("Falha ao iniciar VM do Lua");
    repl::run_loop(&data_base, &lua_engine);
}