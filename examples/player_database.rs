use serde_json::{Value, json};

#[path = "../src/postgre_sql.rs"]
mod postgre_sql;

use postgre_sql::Database;

fn print_data(player: Option<Value>) {
    if let Some(data) = player {
        println!("Player data: {}", data);
    } else {
        println!("Player not found");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::new()?;

    db.init_data_rows()?;

    let player_data = json!({
        "hp": 100,
        "level": 5,
        "inventory": ["sword", "apple"]
    });

    let name: &str = "Ferris";

    db.insert(name, &player_data)?;

    let mut player = db.select(name)?;

    print_data(player);

    db.delete(name)?;

    player = db.select(name)?;

    print_data(player);

    Ok(())
}
