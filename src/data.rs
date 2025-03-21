use crate::db:: init_db;
use crate::models::User;
use std::error::Error;

pub async fn populate_data() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;
    let collection = db.collection::<User>("users");
    collection.drop().await?;

    let users = vec![
        User {
            id: None,
            name: "Sung Jin-Woo".to_string(),
            age: 32,
        },
        User {
            id: None,
            name: "Cha Hae-In".to_string(),
            age: 28,
        },
        User {
            id: None,
            name: "Go Gun-Hee".to_string(),
            age: 82,
        },
        User {
            id: None,
            name: "Baek Yoon-Ho".to_string(),
            age: 37,
        },
        User {
            id: None,
            name: "Choi Jong-In".to_string(),
            age: 40,
        },
        User {
            id: None,
            name: "Woo Jin-Chul".to_string(),
            age: 35,
        },
        User {
            id: None,
            name: "Thomas Andre".to_string(),
            age: 45,
        },
        User {
            id: None,
            name: "Lennart Niermann".to_string(),
            age: 38,
        },
        User {
            id: None,
            name: "Christopher Reed".to_string(),
            age: 42,
        },
        User {
            id: None,
            name: "Liu Zhigang".to_string(),
            age: 39,
        },
        User {
            id: None,
            name: "Goto Ryuji".to_string(),
            age: 41,
        },
        User {
            id: None,
            name: "Ryuji Goto".to_string(),
            age: 41,
        },
        User {
            id: None,
            name: "Hwang Dong-Su".to_string(),
            age: 30,
        },
        User {
            id: None,
            name: "Jinah Sung".to_string(),
            age: 18,
        },
        User {
            id: None,
            name: "Ashborn".to_string(),
            age: 999,
        },
        User {
            id: None,
            name: "Antares".to_string(),
            age: 999,
        },
        User {
            id: None,
            name: "Igris".to_string(),
            age: 300,
        },
        User {
            id: None,
            name: "Beru".to_string(),
            age: 250,
        },
        User {
            id: None,
            name: "Tusk".to_string(),
            age: 275,
        },
        User {
            id: None,
            name: "Iron".to_string(),
            age: 200,
        },
        User {
            id: None,
            name: "Jima".to_string(),
            age: 230,
        },
        User {
            id: None,
            name: "Kamish".to_string(),
            age: 1000,
        },
        User {
            id: None,
            name: "Esil Radir".to_string(),
            age: 20,
        },
        User {
            id: None,
            name: "Norma Selner".to_string(),
            age: 60,
        },
        User {
            id: None,
            name: "Jonas".to_string(),
            age: 32,
        },
        User {
            id: None,
            name: "Emma Laurent".to_string(),
            age: 28,
        },
        User {
            id: None,
            name: "Kang Taeshik".to_string(),
            age: 40,
        },
        User {
            id: None,
            name: "Lee Joohee".to_string(),
            age: 27,
        },
        User {
            id: None,
            name: "Park Kyung-Hye".to_string(),
            age: 50,
        },
        User {
            id: None,
            name: "Hwang Dongsuk".to_string(),
            age: 35,
        },
        User {
            id: None,
            name: "Mitsuyoshi".to_string(),
            age: 42,
        },
        User {
            id: None,
            name: "Ryuji".to_string(),
            age: 39,
        },
        User {
            id: None,
            name: "Osborne".to_string(),
            age: 46,
        },
        User {
            id: None,
            name: "Bellion".to_string(),
            age: 999,
        },
        User {
            id: None,
            name: "Arsha".to_string(),
            age: 310,
        },
        User {
            id: None,
            name: "Erebus".to_string(),
            age: 400,
        },
        User {
            id: None,
            name: "Plutus".to_string(),
            age: 380,
        },
        User {
            id: None,
            name: "Ingrid".to_string(),
            age: 32,
        },
        User {
            id: None,
            name: "Edward Wolker".to_string(),
            age: 44,
        },
        User {
            id: None,
            name: "Hank Jansen".to_string(),
            age: 38,
        },
        User {
            id: None,
            name: "Marcus".to_string(),
            age: 41,
        },
        User {
            id: None,
            name: "Felix".to_string(),
            age: 36,
        },
        User {
            id: None,
            name: "Julian".to_string(),
            age: 33,
        },
        User {
            id: None,
            name: "Leonard".to_string(),
            age: 47,
        },
        User {
            id: None,
            name: "Hans".to_string(),
            age: 40,
        },
        User {
            id: None,
            name: "Carlos".to_string(),
            age: 31,
        },
        User {
            id: None,
            name: "Dante".to_string(),
            age: 29,
        },
        User {
            id: None,
            name: "Vulcan".to_string(),
            age: 50,
        },
        User {
            id: None,
            name: "Maximilian".to_string(),
            age: 43,
        },
        User {
            id: None,
            name: "Yami Sukihero".to_string(),
            age: 43,
        },
    ];

    let result = collection.insert_many(users).await?;
    println!("Inserted {} users into the database", result.inserted_ids.len());
    Ok(())
}
