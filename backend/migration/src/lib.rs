pub use sea_orm_migration::prelude::*;

mod m20250723_235336_create_notes_metadata_table;
mod m20250724_002939_create_comments_table;
mod m20250724_013111_create_visitor_profiles_table;
mod m20250724_014021_create_friends_links_table;
mod m20250724_015502_create_likes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250723_235336_create_notes_metadata_table::Migration),
            Box::new(m20250724_002939_create_comments_table::Migration),
            Box::new(m20250724_013111_create_visitor_profiles_table::Migration),
            Box::new(m20250724_014021_create_friends_links_table::Migration),
            Box::new(m20250724_015502_create_likes_table::Migration),
        ]
    }
}
