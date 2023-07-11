use std::sync::Arc;

use my_no_sql_data_writer::{CreateTableParams, MyNoSqlDataWriter};
use rust_extensions::AppStates;

use crate::{
    my_no_sql::{
        ca_entity::CaMyNoSqlEntity, cert_entity::CertMyNoSqlEntity,
        current_serial_number_entity::CurrentSerialNumberMyNoSqlEntity,
    },
    settings::SettingsReader,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings_reader: Arc<SettingsReader>,
    pub ca_my_no_sql_writer: MyNoSqlDataWriter<CaMyNoSqlEntity>,
    pub ca_serial_number: MyNoSqlDataWriter<CurrentSerialNumberMyNoSqlEntity>,
    pub certs_my_no_sql_writer: MyNoSqlDataWriter<CertMyNoSqlEntity>,
}

impl AppContext {
    pub fn new(settings_reader: Arc<SettingsReader>) -> Self {
        Self {
            ca_my_no_sql_writer: MyNoSqlDataWriter::new(
                settings_reader.clone(),
                CreateTableParams {
                    persist: true,
                    max_partitions_amount: None,
                    max_rows_per_partition_amount: None,
                }
                .into(),
                my_no_sql_server_abstractions::DataSynchronizationPeriod::Sec5,
            ),

            certs_my_no_sql_writer: MyNoSqlDataWriter::new(
                settings_reader.clone(),
                CreateTableParams {
                    persist: true,
                    max_partitions_amount: None,
                    max_rows_per_partition_amount: None,
                }
                .into(),
                my_no_sql_server_abstractions::DataSynchronizationPeriod::Sec5,
            ),

            ca_serial_number: MyNoSqlDataWriter::new(
                settings_reader.clone(),
                CreateTableParams {
                    persist: true,
                    max_partitions_amount: None,
                    max_rows_per_partition_amount: None,
                }
                .into(),
                my_no_sql_server_abstractions::DataSynchronizationPeriod::Sec5,
            ),

            app_states: Arc::new(AppStates::create_initialized()),
            settings_reader,
        }
    }
}
