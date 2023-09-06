#[cfg(test)]
mod test {
    use crate::repository::prelude::*;
    use crate::repository::work_items::WorkItemRequest;
    use actix_web::web;
    use actix_web::web::Json;
    use dotenv::dotenv;
    use sea_orm::{Database, DatabaseConnection};

    #[async_std::test]
    async fn should_create_new_work_item_and_then_delete_works() {
        dotenv().ok();
        let repository = WorkItemRepository {
            db: Database::connect(std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        };
        let wir = WorkItemRequest {
            title: "Cogntive Complexity Çalışması".to_string(),
            summary: "Modüldeki Complexity değeri 100 üstünde olan fonksiyonların iyileştirilmesi"
                .to_string(),
            business_value: 300,
            completed: false,
        };
        let model = repository.create_work_item(Json(wir)).await.unwrap();

        let inserted = repository.get_work_item_by_id(model.id).await.unwrap();
        assert_eq!(inserted.title, "Cogntive Complexity Çalışması".to_string());

        let deleted_row = repository.delete_work_item(inserted.id).await.rows_affected;
        assert_eq!(deleted_row, 1);
    }
}
