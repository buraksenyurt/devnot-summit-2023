use crate::entity::{prelude::*, work_item};
use actix_web::web::Json;
use log::debug;
use sea_orm::{entity::*, DeleteResult};
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemRequest {
    pub title: String,
    pub summary: String,
    pub business_value: i32,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct WorkItemRepository {
    pub db: DatabaseConnection,
}

impl WorkItemRepository {
    pub async fn get_work_items(&self) -> Vec<work_item::Model> {
        WorkItem::find()
            .all(&self.db)
            .await
            .expect("WorkItem listesi çekilirken hata oluştu.")
    }

    pub async fn get_work_item_by_id(&self, id: i32) -> Option<work_item::Model> {
        WorkItem::find_by_id(id)
            .one(&self.db)
            .await
            .expect("Work Item çekilirken hata oluştu")
    }

    pub async fn create_work_item(
        &self,
        payload: Json<WorkItemRequest>,
    ) -> Option<work_item::Model> {
        let work_item = work_item::ActiveModel {
            id: NotSet,
            title: ActiveValue::Set(payload.title.to_owned()),
            completed: ActiveValue::Set(payload.completed.to_owned()),
            summary: ActiveValue::Set(payload.summary.to_owned()),
            business_value: ActiveValue::Set(payload.business_value.to_owned()),
        };
        let work_item: work_item::Model = work_item.insert(&self.db).await.unwrap();
        debug!("Work Item oluşturuldu {}", work_item.title);
        work_item.into()
    }

    pub async fn update_work_item(
        &self,
        id: i32,
        payload: Json<WorkItemRequest>,
    ) -> Option<work_item::Model> {
        let work_item = WorkItem::find_by_id(id)
            .one(&self.db)
            .await
            .expect("Work Item bulunamadı");
        let mut work_item: work_item::ActiveModel = work_item.unwrap().into();
        work_item.title = ActiveValue::Set(payload.title.to_owned());
        work_item.completed = ActiveValue::Set(payload.completed.to_owned());
        work_item.summary = ActiveValue::Set(payload.summary.to_owned());
        work_item.business_value = ActiveValue::Set(payload.business_value.to_owned());

        let work_item: work_item::Model = work_item.update(&self.db).await.unwrap();
        debug!("Work Item güncellendi. {}", work_item.title);
        work_item.into()
    }

    pub async fn delete_work_item(&self, id: i32) -> DeleteResult {
        let result: DeleteResult = WorkItem::delete_by_id(id).exec(&self.db).await.unwrap();
        result
    }
}
