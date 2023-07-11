use serde::{Deserialize, Serialize};

pub const ROW_KEY: &str = "current";
#[my_no_sql_macros::my_no_sql_entity("cert")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentSerialNumberMyNoSqlEntity {
    pub id: u32,
}

impl CurrentSerialNumberMyNoSqlEntity {
    pub fn new(cn: String) -> Self {
        Self {
            partition_key: cn,
            row_key: ROW_KEY.to_string(),
            time_stamp: "".to_string(),
            id: 0,
        }
    }
}
