use crosup_entity::modification;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

pub struct ModificationRepo {
    db: DatabaseConnection,
}

impl ModificationRepo {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find(&self, id: i32) -> Result<Option<modification::Model>, DbErr> {
        modification::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_hash(&self, hash: &str) -> Result<Option<modification::Model>, DbErr> {
        modification::Entity::find()
            .filter(modification::Column::Hash.eq(hash))
            .one(&self.db)
            .await
    }

    pub async fn find_by_file_id(&self, file_id: i32) -> Result<Vec<modification::Model>, DbErr> {
        modification::Entity::find()
            .filter(modification::Column::FileId.eq(file_id))
            .order_by_desc(modification::Column::Timestamp)
            .all(&self.db)
            .await
    }

    pub async fn find_last_by_file_id(
        &self,
        file_id: i32,
    ) -> Result<Option<modification::Model>, DbErr> {
        modification::Entity::find()
            .filter(modification::Column::FileId.eq(file_id))
            .order_by_desc(modification::Column::Timestamp)
            .one(&self.db)
            .await
    }

    pub async fn create(
        &self,
        file_id: i32,
        hash: &str,
        content: &str,
    ) -> Result<modification::Model, DbErr> {
        let result = self.find_by_hash(hash).await?;

        if result.is_some() {
            return Ok(result.unwrap());
        }

        let last_modification = self.find_last_by_file_id(file_id).await?;
        let previous_id = last_modification.map(|m| m.id);

        modification::ActiveModel {
            file_id: Set(file_id),
            hash: Set(hash.to_owned()),
            content: Set(content.to_owned()),
            previous_id: Set(previous_id),
            ..Default::default()
        }
        .insert(&self.db)
        .await
    }
}
