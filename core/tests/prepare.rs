use ::entity::address;
use sea_orm::*;

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
  MockDatabase::new(DatabaseBackend::Postgres)
    .append_query_results(vec![
      vec![address::Model {
        id: 106137538,
        fullname: "Sporta iela 18, Rīga".to_owned(),
        search_text_name: "sporta iela 18, riga".to_owned(),
        full_code: "100000000.100003003.100312001.101996287".to_owned(),
        postal_index: "LV-2113".to_owned(),
      }],
      vec![address::Model {
        id: 4493228686,
        full_code: "18133.4489960724.4489960754.4490063534.4490137680.4491923597.4493228686".to_owned(),
        fullname: "Trakų g. 34-11, Šiauliai".to_owned(),
        search_text_name: "traku g. 34-11, siauliai".to_owned(),
        postal_index: "LT-76292".to_owned(),
      }],
      vec![address::Model {
        id: 4491273809,
        full_code: "18133.4489960730.4489960756.4490092133.4490202204.4491273809".to_owned(),
        fullname: "Elektrinės g. 34-11, Drūkšinių k.".to_owned(),
        search_text_name: "elektrines g. 34-11, druksiniu k..to_owned()".to_owned(),
        postal_index: "".to_owned(),
      }],
      vec![address::Model {
        id: 4493547517,
        full_code: "18133.4489960718.4489960746.4490091581.4490199004.4491515147.4493547517".to_owned(),
        fullname: "Paryžiaus Komunos g. 22-17, Klaipėda".to_owned(),
        search_text_name: "paryziaus komunos g. 22-17, klaipeda".to_owned(),
        postal_index: "LT-91171".to_owned(),
      }],
      vec![address::Model {
        id: 4494201844,
        full_code: "18133.4489960732.4489960770.4490025096.4490086286.4490187174.4491055001.4494201844".to_owned(),
        fullname: "Šv. Jono g. 7-3, Izabelinės k., Sudervės sen., Vilniaus rajono sav.".to_owned(),
        search_text_name: "sv. jono g. 7-3, izabelines k., suderves sen., vilniaus rajono sav.".to_owned(),
        postal_index: "".to_owned(),
      }],
      vec![address::Model {
        id: 1,
        full_code: "18133.4489960718.4489960746.4490091581.4490199004.4491515147.4493547517".to_owned(),
        fullname: "Šv. Jono g. 7-3, Izabelinės k., Sudervės sen., Klaipeda rajono sav.".to_owned(),
        search_text_name: "sv. jono g. 7-3, izabelines k., suderves sen., klaipeda rajono sav.".to_owned(),
        postal_index: "LT-91171".to_owned(),
      }],
      vec![address::Model {
        id: 4493242177,
        full_code: "18133.4489960732.4489960738.4490063513.4490162892.4492123310.4493242177".to_owned(),
        fullname: "Perkūnkiemio g. 53-98, Vilnius".to_owned(),
        search_text_name: "perkunkiemio g. 53-98, vilnius".to_owned(),
        postal_index:  "LT-12144".to_owned(),
      }],
      vec![address::Model {
        id: 4492757824,
        full_code: "18133.4489960732.4489960738.4490063513.4490162892.4492113344.4492757824".to_owned(),
        fullname: "Perkūnkiemio g. 10-15, Vilnius".to_owned(),
        search_text_name: "perkunkiemio g. 10-15, vilnius".to_owned(),
        postal_index: "LT-12113".to_owned(),
      }],
      vec![address::Model {
        id: 4492122510,
        full_code: "18133.4489960732.4489960738.4490063513.4490162892.4492122510".to_owned(),
        fullname: "Perkūnkiemio g. 5, Vilnius".to_owned(),
        search_text_name: "perkunkiemio g. 5, vilnius".to_owned(),
        postal_index:  "LT-12129".to_owned(),
      }],
      vec![address::Model {
        id: 849997196,
        full_code: "18180.2248909776.2248909847.6847016130.6849946708.6849997196".to_owned(),
        fullname: "ul. Pamiętna, Warszawa, Wilanów, Warszawa, mazowieckie".to_owned(),
        search_text_name: "ul. pamietna, warszawa, wilanow, warszawa, mazowieckie".to_owned(),
        postal_index: "02-972".to_owned(),
      }],
      vec![address::Model {
        id: 16082246409,
        full_code: "100000000.100003003.100308307.102649726.16082246409".to_owned(),
        fullname: "Mazā Nometņu iela 1-12A, Rīga".to_owned(),
        search_text_name: "maza nometnu iela 1-12a, riga".to_owned(),
        postal_index: "LV-1021".to_owned(),
      }],
      vec![address::Model {
        id: 2248955269,
        full_code: "18180.2248909765.2248910145.2248913708.2248955269".to_owned(),
        fullname: "Darłowo, sławieński, zachodniopomorskie".to_owned(),
        search_text_name: "darlowo, slawienski, zachodniopomorskie".to_owned(),
        postal_index: "76-150".to_owned(),
      }],
      vec![address::Model {
        id: 2249088796,
        full_code: "18180.2248909765.2248910145.2248913708.2248955269.2249088796".to_owned(),
        fullname: "ul. Bogusława X, Darłowo, sławieński, zachodniopomorskie".to_owned(),
        search_text_name: "ul. boguslawa x, darlowo, slawienski, zachodniopomorskie".to_owned(),
        postal_index: "76-150".to_owned(),
      }],
      vec![address::Model {
        id: 2249045984,
        full_code: "18180.2248909761.2248909836.2248914123.2248959588.2249045984".to_owned(),
        fullname: "ul. Biały Potok, Krościenko nad Dunajcem, nowotarski, małopolskie".to_owned(),
        search_text_name: "ul. bialy potok, kroscienko nad dunajcem, nowotarski, malopolskie".to_owned(),
        postal_index: "34-450".to_owned(),
      }],
    ])
    .into_connection()
}

