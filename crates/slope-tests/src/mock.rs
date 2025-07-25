use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::context::Context;
use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::response::CallResponse;
use anyhow::{anyhow, Result};
use bitcoin::Txid;
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock implementation of the Context for testing
pub struct MockContext {
    pub inputs: Vec<u128>,
    pub myself: AlkaneId,
    pub incoming_alkanes: AlkaneTransferParcel,
    pub transaction_id: Txid,
    pub caller: AlkaneId,
    pub vout: u32,
}

impl Default for MockContext {
    fn default() -> Self {
        Self {
            inputs: Vec::new(),
            myself: AlkaneId::default(),
            incoming_alkanes: AlkaneTransferParcel::default(),
            transaction_id: Txid::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            caller: AlkaneId::default(),
            vout: 0,
        }
    }
}

/// Mock implementation of the AlkaneResponder trait for testing
pub struct MockAlkaneResponder {
    pub context: MockContext,
    pub storage: Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl Default for MockAlkaneResponder {
    fn default() -> Self {
        Self {
            context: MockContext::default(),
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MockAlkaneResponder {
    pub fn new(context: MockContext) -> Self {
        Self {
            context,
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_transaction_id(mut self, txid: Txid) -> Self {
        self.context.transaction_id = txid;
        self
    }

    pub fn with_inputs(mut self, inputs: Vec<u128>) -> Self {
        self.context.inputs = inputs;
        self
    }

    pub fn with_storage_value(self, key: &[u8], value: &[u8]) -> Self {
        self.storage.lock().unwrap().insert(key.to_vec(), value.to_vec());
        self
    }

    pub fn get_storage_value(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.lock().unwrap().get(key).cloned()
    }
}

impl AlkaneResponder for MockAlkaneResponder {
    fn context(&self) -> Result<Context> {
        Ok(Context {
            inputs: self.context.inputs.clone(),
            myself: self.context.myself.clone(),
            incoming_alkanes: self.context.incoming_alkanes.clone(),
            caller: self.context.caller.clone(),
            vout: self.context.vout,
        })
    }

    fn transaction(&self) -> Vec<u8> {
        Vec::new() // Mock implementation
    }

    fn load(&self, k: Vec<u8>) -> Vec<u8> {
        self.storage.lock().unwrap().get(&k).cloned().unwrap_or_default()
    }

    fn store(&self, k: Vec<u8>, v: Vec<u8>) {
        self.storage.lock().unwrap().insert(k, v);
    }
}

/// Extension trait for Context to add transaction_id method
pub trait ContextExt {
    /// Get the transaction ID from the context
    fn transaction_id(&self) -> Result<Txid>;
}

impl ContextExt for Context {
    fn transaction_id(&self) -> Result<Txid> {
        // In a real implementation, this would extract the transaction ID from the context
        // For testing, we'll use a placeholder implementation with all zeros
        Ok(Txid::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap())
    }
}

/// Mock implementation of StoragePointer for testing
pub struct MockStoragePointer {
    key: String,
    storage: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl MockStoragePointer {
    pub fn from_keyword(key: &str) -> Self {
        static GLOBAL_STORAGE: once_cell::sync::Lazy<Arc<Mutex<HashMap<String, Vec<u8>>>>> = 
            once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
        
        Self {
            key: key.to_string(),
            storage: GLOBAL_STORAGE.clone(),
        }
    }

    pub fn get(&self) -> Arc<Vec<u8>> {
        let storage = self.storage.lock().unwrap();
        Arc::new(storage.get(&self.key).cloned().unwrap_or_default())
    }

    pub fn set(&mut self, value: Arc<Vec<u8>>) {
        let mut storage = self.storage.lock().unwrap();
        storage.insert(self.key.clone(), value.as_ref().clone());
    }

    pub fn get_value<T: Default + From<Vec<u8>>>(&self) -> T {
        let value = self.get();
        if value.len() == 0 {
            T::default()
        } else {
            T::from(value.as_ref().clone())
        }
    }

    pub fn set_value<T: Into<Vec<u8>>>(&self, value: T) {
        let mut this = self.clone();
        this.set(Arc::new(value.into()));
    }
}

impl Clone for MockStoragePointer {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            storage: self.storage.clone(),
        }
    }
}
