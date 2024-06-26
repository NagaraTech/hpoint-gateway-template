//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "relay_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub event_type: String,
    pub time_stamp: DateTimeWithTimeZone,
    pub address: String,
    pub project_name: String,
    pub sign: String,
    pub event_date: Date,
    pub duration: Option<i32>,
    pub is_sent: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
