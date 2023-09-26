use datafusion::common::DataFusionError;
use datafusion::common::Result;
use datafusion::datasource::object_store::ObjectStoreRegistry;
use datafusion_objectstore_hdfs::object_store::hdfs::HadoopFileSystem;
use object_store::local::LocalFileSystem;
use object_store::ObjectStore;
use std::sync::Arc;
use url::Url;

#[derive(Debug)]
pub struct ObjectStoreRegistryImpl;

impl ObjectStoreRegistry for ObjectStoreRegistryImpl {
    fn register_store(
        &self,
        _url: &Url,
        _store: Arc<dyn ObjectStore>,
    ) -> Option<Arc<dyn ObjectStore>> {
        unreachable!()
    }

    fn get_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>> {
        match url.scheme() {
            "hdfs" => build_hdfs_object_store(url),
            "file" => Ok(Arc::new(LocalFileSystem::new()) as Arc<dyn object_store::ObjectStore>),
            _ => Err(DataFusionError::Execution(format!(
                "Unsupported object store scheme: [{}]",
                url.scheme()
            ))),
        }
    }
}

fn build_hdfs_object_store(url: &Url) -> Result<Arc<dyn ObjectStore>> {
    let store = HadoopFileSystem::new(url.as_str());
    if let Some(store) = store {
        return Ok(Arc::new(store));
    }

    Err(DataFusionError::Execution(format!(
        "No object store provider found for url: {}",
        url
    )))
}
