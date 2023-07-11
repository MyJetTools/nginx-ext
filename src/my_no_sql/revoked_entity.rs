use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("cas")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokedMyNoSqlEntity {
    pub revoked: Vec<u32>,
}

impl RevokedMyNoSqlEntity {
    pub fn get_row_key() -> &'static str {
        "revoked"
    }

    pub fn new(ca_cn: &str) -> Self {
        Self {
            partition_key: ca_cn.to_string(),
            row_key: Self::get_row_key().to_string(),
            time_stamp: "".to_string(),
            revoked: vec![],
        }
    }
}
