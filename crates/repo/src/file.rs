use crosup_entity::file;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

pub struct FileRepo {
    db: DatabaseConnection,
}

impl FileRepo {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find(&self, id: i32) -> Result<Option<file::Model>, DbErr> {
        file::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_path(&self, path: &str) -> Result<Option<file::Model>, DbErr> {
        file::Entity::find()
            .filter(file::Column::Path.eq(path))
            .one(&self.db)
            .await
    }

    pub async fn create(&self, name: &str, path: &str) -> Result<Option<file::Model>, DbErr> {
        let result = self.find_by_path(path).await?;

        if let Some(_) = result {
            return Ok(None);
        }

        Ok(Some(
            file::ActiveModel {
                name: Set(name.to_owned()),
                path: Set(path.to_owned()),
                ..Default::default()
            }
            .insert(&self.db)
            .await?,
        ))
    }
}
