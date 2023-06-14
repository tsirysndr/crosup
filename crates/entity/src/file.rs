use sea_orm::entity::prelude::*;
use sea_orm::DeriveEntityModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub path: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::modification::Entity")]
    Modification,
}

// `Related` trait has to be implemented by hand
impl Related<super::modification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Modification.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
