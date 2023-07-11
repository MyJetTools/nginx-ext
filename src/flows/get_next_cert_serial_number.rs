use crate::{
    app::AppContext, my_no_sql::current_serial_number_entity::CurrentSerialNumberMyNoSqlEntity,
};

pub async fn get_next_cert_serial_number(app: &AppContext, common_name: &str) -> u32 {
    let entity = app
        .ca_serial_number
        .get_entity(
            common_name,
            crate::my_no_sql::current_serial_number_entity::ROW_KEY,
            None,
        )
        .await
        .unwrap();

    let mut entity = if let Some(entity) = entity {
        entity
    } else {
        CurrentSerialNumberMyNoSqlEntity::new(common_name.to_string())
    };

    entity.id += 1;

    app.ca_serial_number.insert_or_replace_entity(&entity).await;

    entity.id
}
