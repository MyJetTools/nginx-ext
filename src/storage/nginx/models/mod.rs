mod upstreams;
use std::collections::HashMap;

use serde::*;
pub use upstreams::*;

#[derive(Default, Deserialize, Serialize)]
pub struct NginxFileContent {
    pub upstreams: HashMap<String, Vec<UpStreamRouteStorageModel>>,
}
