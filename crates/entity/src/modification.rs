use sea_orm::entity::prelude::*;
use sea_orm::DeriveEntityModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "modification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub timestamp: chrono::NaiveDateTime,
    pub hash: String,
    pub file_id: i32,
    pub previous_id: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    File,
    Modification,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::File => Entity::belongs_to(super::file::Entity)
                .from(Column::FileId)
                .to(super::file::Column::Id)
                .into(),
            Self::Modification => Entity::has_one(super::modification::Entity)
                .from(Column::Id)
                .to(super::modification::Column::PreviousId)
                .into(),
        }
    }
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

impl Related<super::modification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Modification.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
