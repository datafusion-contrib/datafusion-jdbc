use dashmap::DashMap;
use datafusion::common::DataFusionError;
use datafusion::common::Result;
use datafusion::datasource::object_store::ObjectStoreRegistry;
use deltalake::storage::config::{configure_store, StorageOptions};
use deltalake::ObjectStore;
use std::sync::Arc;
use url::Url;

#[derive(Debug)]
pub struct ObjectStoreRegistryImpl {
    object_stores: DashMap<String, Arc<dyn ObjectStore>>,
}

impl ObjectStoreRegistryImpl {
    pub fn new() -> Self {
        Self {
            object_stores: DashMap::new(),
        }
    }
}

impl ObjectStoreRegistry for ObjectStoreRegistryImpl {
    fn register_store(
        &self,
        url: &Url,
        store: Arc<dyn ObjectStore>,
    ) -> Option<Arc<dyn ObjectStore>> {
        let s = get_url_key(url);
        self.object_stores.insert(s, store)
    }

    fn get_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>> {
        let key = get_url_key(url);
        if let Some(store) = self.object_stores.get(&key) {
            return Ok(store.value().clone());
        } else {
            configure_store(url, &mut StorageOptions::default()).map_err(|e| {
                DataFusionError::Execution(format!(
                    "No object store provider found for url: {}, error: {:?}",
                    url, e
                ))
            })
        }
    }
}

/// Get the key of a url for object store registration.
/// The credential info will be removed
fn get_url_key(url: &Url) -> String {
    format!(
        "{}://{}",
        url.scheme(),
        &url[url::Position::BeforeHost..url::Position::AfterPort],
    )
}
