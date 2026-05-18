use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Repo::Table)
                    .if_not_exists()
                    .col(pk_auto(Repo::Id))
                    .col(string(Repo::Name))
                    .col(string(Repo::Path))
                    .col(string(Repo::RemoteUrl).null())
                    .col(timestamp(Repo::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Username))
                    .col(string(User::EMail))
                    .col(timestamp(User::CreatedAt))
                    .col(timestamp(User::LastLogin).null())
                    .col(string(User::Role))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .if_not_exists()
                    .col(pk_auto(Author::Id))
                    .col(integer(Author::UserId).null())
                    .col(string(Author::EMail))
                    .col(string(Author::Name))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_author_user")
                            .from(Author::Table, Author::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommitMeta::Table)
                    .if_not_exists()
                    .col(string(CommitMeta::HashId).primary_key())
                    .col(string(CommitMeta::Author))
                    .col(text(CommitMeta::Message))
                    .col(timestamp(CommitMeta::Timestamp))
                    .col(string(CommitMeta::Refs).null())
                    .col(integer(CommitMeta::RepoId))
                    .col(integer(CommitMeta::AuthorId).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commit_repo")
                            .from(CommitMeta::Table, CommitMeta::RepoId)
                            .to(Repo::Table, Repo::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commit_author")
                            .from(CommitMeta::Table, CommitMeta::AuthorId)
                            .to(Author::Table, Author::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Issue::Table)
                    .if_not_exists()
                    .col(pk_auto(Issue::Id))
                    .col(string(Issue::Title))
                    .col(text(Issue::Description))
                    .col(string(Issue::Status))
                    .col(string(Issue::Priority))
                    .col(timestamp(Issue::CreatedAt))
                    .col(timestamp(Issue::UpdatedAt))
                    .col(integer(Issue::AssigneeId).null())
                    .col(integer(Issue::ReporterId))
                    .col(integer(Issue::RepoId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_assignee")
                            .from(Issue::Table, Issue::AssigneeId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_reporter")
                            .from(Issue::Table, Issue::ReporterId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_repo")
                            .from(Issue::Table, Issue::RepoId)
                            .to(Repo::Table, Repo::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Branch::Table)
                    .if_not_exists()
                    .col(pk_auto(Branch::Id))
                    .col(integer(Branch::RepoId))
                    .col(string(Branch::Name))
                    .col(boolean(Branch::IsDefault).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_branch_repo")
                            .from(Branch::Table, Branch::RepoId)
                            .to(Repo::Table, Repo::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(pk_auto(Tag::Id))
                    .col(string(Tag::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommitTag::Table)
                    .if_not_exists()
                    .col(integer(CommitTag::TagId))
                    .col(string(CommitTag::CommitHashId))
                    .primary_key(
                        Index::create()
                            .col(CommitTag::TagId)
                            .col(CommitTag::CommitHashId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commit_tag_tag")
                            .from(CommitTag::Table, CommitTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commit_tag_commit")
                            .from(CommitTag::Table, CommitTag::CommitHashId)
                            .to(CommitMeta::Table, CommitMeta::HashId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssueComment::Table)
                    .if_not_exists()
                    .col(pk_auto(IssueComment::Id))
                    .col(integer(IssueComment::IssueId))
                    .col(integer(IssueComment::AuthorId))
                    .col(text(IssueComment::Body))
                    .col(timestamp(IssueComment::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_comment_issue")
                            .from(IssueComment::Table, IssueComment::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_comment_author")
                            .from(IssueComment::Table, IssueComment::AuthorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommitIssue::Table)
                    .if_not_exists()
                    .col(integer(CommitIssue::IssueId))
                    .col(string(CommitIssue::CommitHashId))
                    .col(string(CommitIssue::RelationType))
                    .primary_key(
                        Index::create()
                            .col(CommitIssue::IssueId)
                            .col(CommitIssue::CommitHashId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commit_issue_issue")
                            .from(CommitIssue::Table, CommitIssue::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commit_issue_commit")
                            .from(CommitIssue::Table, CommitIssue::CommitHashId)
                            .to(CommitMeta::Table, CommitMeta::HashId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssueEvent::Table)
                    .if_not_exists()
                    .col(pk_auto(IssueEvent::Id))
                    .col(integer(IssueEvent::IssueId))
                    .col(string(IssueEvent::EventType))
                    .col(timestamp(IssueEvent::CreatedAt))
                    .col(integer(IssueEvent::ActorId).null())
                    .col(integer(IssueEvent::CommentId).null())
                    .col(string(IssueEvent::CommitHashId).null())
                    .col(string(IssueEvent::FromStatus).null())
                    .col(string(IssueEvent::ToStatus).null())
                    .col(integer(IssueEvent::AssigneeId).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_event_issue")
                            .from(IssueEvent::Table, IssueEvent::IssueId)
                            .to(Issue::Table, Issue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_event_actor")
                            .from(IssueEvent::Table, IssueEvent::ActorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_event_comment")
                            .from(IssueEvent::Table, IssueEvent::CommentId)
                            .to(IssueComment::Table, IssueComment::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_event_commit")
                            .from(IssueEvent::Table, IssueEvent::CommitHashId)
                            .to(CommitMeta::Table, CommitMeta::HashId)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_issue_event_assignee")
                            .from(IssueEvent::Table, IssueEvent::AssigneeId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(IssueEvent::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(CommitIssue::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(IssueComment::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(CommitTag::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tag::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Branch::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Issue::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(CommitMeta::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Author::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Repo::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Issue {
    Table,
    Id,
    Title,
    Description,
    Status,
    Priority,
    CreatedAt,
    UpdatedAt,
    AssigneeId,
    ReporterId,
    RepoId,
}

#[derive(DeriveIden)]
enum CommitMeta {
    Table,
    HashId,
    Author,
    Message,
    Timestamp,
    Refs,
    RepoId,
    AuthorId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    EMail,
    CreatedAt,
    LastLogin,
    Role,
}

#[derive(DeriveIden)]
enum Author {
    Table,
    Id,
    UserId,
    EMail,
    Name,
}

#[derive(DeriveIden)]
enum Repo {
    Table,
    Id,
    Name,
    Path,
    RemoteUrl,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Branch {
    Table,
    Id,
    RepoId,
    Name,
    IsDefault,
}

#[derive(DeriveIden)]
enum IssueComment {
    Table,
    Id,
    IssueId,
    AuthorId,
    Body,
    CreatedAt,
}

#[derive(DeriveIden)]
enum CommitIssue {
    Table,
    IssueId,
    CommitHashId,
    RelationType,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum CommitTag {
    Table,
    TagId,
    CommitHashId,
}

#[derive(DeriveIden)]
enum IssueEvent {
    Table,
    Id,
    IssueId,
    EventType,
    CreatedAt,
    ActorId,
    CommentId,
    CommitHashId,
    FromStatus,
    ToStatus,
    AssigneeId,
}
