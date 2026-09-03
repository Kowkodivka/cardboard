use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("boards")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(string_len("slug", 32).unique_key())
                    .col(string_len("name", 64))
                    .col(string_len_null("description", 2_000))
                    .col(big_integer("thread_limit").default(100))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(binary_len("salt", 16))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("daily_salt")
                    .if_not_exists()
                    .col(integer("id").primary_key().default(1))
                    .col(binary_len("value", 16))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("posts")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("board_id"))
                    .col(uuid("root_post_id"))
                    .col(uuid_null("parent_post_id"))
                    .col(string_len("author_tripcode", 16))
                    .col(text("content"))
                    .col(integer("reply_count").default(0))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone_null("last_bumped_at"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_board_id")
                            .from("posts", "board_id")
                            .to("boards", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_parent_post_id")
                            .from("posts", "parent_post_id")
                            .to("posts", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_board_id")
                    .table("posts")
                    .col("board_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_parent_post_id")
                    .table("posts")
                    .col("parent_post_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_created_at")
                    .table("posts")
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_root_created")
                    .table("posts")
                    .col("root_post_id")
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("posts").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("daily_salt").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("boards").to_owned())
            .await?;

        Ok(())
    }
}
