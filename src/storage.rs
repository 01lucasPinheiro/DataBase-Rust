use std::collections::HashMap;
use std::sync::RwLock;

pub struct CacheDatabase{
    store: RwLock<HashMap<String, String>>,
}
impl CacheDatabase {
    pub fn new()-> Self{
        CacheDatabase { store: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String>{
        let store = self.store.read().unwrap();
        store.get(key).cloned() 
    }

    pub fn set(&self, key: String, value: String){
        let mut store = self.store.write().unwrap();
        store.insert(key, value);
    }

    pub fn delete(&self, key: &str) -> Option<String>{
        let mut store = self.store.write().unwrap();
        store.remove(key)
    }

    pub fn find_by_value(&self, search_value: &str) -> Option<String> {
        let store = self.store.read().unwrap(); // Trava apenas para leitura
        for (k, v) in store.iter() {
            if v == search_value {
                return Some(k.clone());
            }
        }
        None
    }
}
