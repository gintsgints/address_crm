mod prepare;

use crm_address_core::{Query};
use prepare::prepare_mock_db;

#[tokio::test]
async fn main() {
  let db = &prepare_mock_db();

  {
    let page = Query::find_address_in_page(db, "elektr".to_owned(), 0, 1).await;
    let page_unwrapped = page.unwrap();
    println!("{:?}", page_unwrapped.0);
    assert!(true)
  }
}
