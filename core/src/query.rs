use ::entity::{address, address::Entity as Address};
use sea_orm::*;

pub struct Query;

impl Query {
  pub async fn find_address_in_page(
    db: &DbConn,
    search: String,
    page: u64,
    posts_per_page: u64,
  ) -> Result<(Vec<address::Model>, u64), DbErr> {
    let paginator = Address::find()
      .filter(address::Column::SearchTextName.contains(search.as_str()))
      .order_by_asc(address::Column::FullCode)
      .paginate(db, posts_per_page);
    let num_pages = paginator.num_pages().await?;

    paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
  }
}
