//! Session management utilities.
//!
//! This module defines the [`SessionStore`] trait used to load and persist
//! session data. A simple file-based implementation [`FileStore`] is provided
//! as the default backend. Developers can implement [`SessionStore`] for their
//! own storage solutions such as databases or key-value stores.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use uuid::Uuid;

use crate::concepts::value::json::JsonFormatter;
use crate::concepts::value::{Value, ValueFormatter};

/// Backend used to load and persist session data.
pub trait SessionStore {
    /// Load all key/value pairs associated with `id`.
    fn load(&self, id: &str) -> HashMap<String, Value>;
    /// Persist all key/value pairs for `id`.
    fn save(&self, id: &str, data: &HashMap<String, Value>);
    /// Remove all data associated with `id`.
    fn delete(&self, id: &str);
}

/// Generate a random, collision-resistant session identifier.
///
/// This uses a UUID v4 under the hood.
///
/// # Examples
/// ```
/// use hermes::http::session::generate_id;
///
/// let id1 = generate_id();
/// let id2 = generate_id();
/// assert_ne!(id1, id2);
/// ```
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Clone)]
/// File-based session store saving each session in a separate file.
pub struct FileStore<F: ValueFormatter + Clone = JsonFormatter> {
    root: PathBuf,
    formatter: F,
}

impl<F: ValueFormatter + Clone> FileStore<F> {
    /// Create a new store saving sessions under `dir` with `formatter`.
    pub fn with_formatter(dir: impl Into<PathBuf>, formatter: F) -> Self {
        let root = dir.into();
        fs::create_dir_all(&root).ok();
        Self { root, formatter }
    }

    fn path(&self, id: &str) -> PathBuf {
        self.root.join(id)
    }
}

impl FileStore<JsonFormatter> {
    /// Create a new store saving sessions under `dir` using a JSON formatter.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self::with_formatter(dir, JsonFormatter)
    }
}

impl<F: ValueFormatter + Clone> SessionStore for FileStore<F> {
    fn load(&self, id: &str) -> HashMap<String, Value> {
        let path = self.path(id);
        let mut data = HashMap::new();
        if let Ok(mut f) = File::open(path) {
            let mut buf = String::new();
            if f.read_to_string(&mut buf).is_ok() {
                for line in buf.lines() {
                    if let Some((k, v)) = line.split_once('=') {
                        if let Some(val) = self.formatter.parse(v) {
                            data.insert(k.to_string(), val);
                        }
                    }
                }
            }
        }
        data
    }

    fn save(&self, id: &str, data: &HashMap<String, Value>) {
        let path = self.path(id);
        if let Ok(mut f) = File::create(path) {
            for (k, v) in data {
                let _ = writeln!(f, "{}={}", k, self.formatter.format(v.clone()));
            }
        }
    }

    fn delete(&self, id: &str) {
        let path = self.path(id);
        let _ = fs::remove_file(path);
    }
}

/// In-memory representation of a session loaded from a store.
pub struct Session<S: SessionStore + Clone> {
    id: String,
    data: HashMap<String, Value>,
    store: S,
}

impl<S: SessionStore + Clone> Session<S> {
    /// Create a session with the given identifier backed by `store`.
    pub fn new(id: impl Into<String>, store: S) -> Self {
        let id = id.into();
        let data = store.load(&id);
        Self { id, data, store }
    }

    /// Retrieve a value from the session.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Insert or update a value in the session.
    pub fn insert(&mut self, key: impl Into<String>, value: Value) {
        self.data.insert(key.into(), value);
    }

    /// Remove a value from the session.
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    /// Persist the current session state using the underlying store.
    pub fn persist(&self) {
        self.store.save(&self.id, &self.data);
    }

    /// Unique identifier of this session.
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::value::Value;
    use std::env;

    #[test]
    fn file_store_round_trip() {
        let dir = env::temp_dir().join("hermes_session_test");
        let store = FileStore::new(&dir);
        let mut data = HashMap::new();
        data.insert("foo".to_string(), Value::String("bar".to_string()));
        store.save("s1", &data);
        let loaded = store.load("s1");
        assert_eq!(loaded.get("foo"), Some(&Value::String("bar".to_string())));
        store.delete("s1");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn session_persists_data() {
        let dir = env::temp_dir().join("hermes_session_persist");
        let store = FileStore::new(&dir);
        {
            let mut sess = Session::new("s2", store.clone());
            sess.insert("a", Value::Int(1));
            sess.persist();
        }
        let store = FileStore::new(&dir);
        let sess = Session::new("s2", store.clone());
        assert_eq!(sess.get("a"), Some(&Value::Int(1)));
        store.delete("s2");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn generate_id_creates_unique_strings() {
        let a = generate_id();
        let b = generate_id();
        assert_ne!(a, b);
        assert_eq!(a.len(), 36);
    }
}
