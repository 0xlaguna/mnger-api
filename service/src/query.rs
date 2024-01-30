use ::entity::{user, user::Entity as User};
use sea_orm::*;

use crate::Result;
use crate::{create_error, create_database_error};

pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<user::Model> {
        let user = User::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| create_database_error!(err))?
            .ok_or_else(|| create_error!(NotFound))?;

        Ok(user)
    }
    
    /// If ok, returns (user models, num pages).
    pub async fn find_users_in_page(
        db: &DbConn,
        page: u64,
        users_per_page: u64,
    ) -> Result<(Vec<user::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = User::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, users_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated users
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
