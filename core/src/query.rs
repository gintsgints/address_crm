use ::entity::{address, address::Entity as Address};
use sea_orm::*;

pub struct Query;

impl Query {
  pub async fn find_address_in_page(
    db: &DbConn,
    search: Vec<String>,
    page: u64,
    posts_per_page: u64,
  ) -> Result<(Vec<address::Model>, ItemsAndPagesNumber), DbErr> {
    let mut conditions = Condition::all();
    for search_str in search {
      conditions = conditions.add(address::Column::SearchTextName.contains(search_str.as_str()));
    }

    let paginator = Address::find()
      .filter(conditions)
      .order_by_asc(address::Column::FullCode)
      .paginate(db, posts_per_page);
    let items_and_pages_number = paginator.num_items_and_pages().await?;

    paginator.fetch_page(page - 1).await.map(|p| (p, items_and_pages_number))
  }
}
