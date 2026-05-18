use serde_json::json;

mod postgre_sql;

use postgre_sql::Database;

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

    let player = db.select(name)?;

    if let Some(data) = player {
        println!("Player data: {}", data);
    } else {
        println!("Player not found");
    }

    Ok(())
}
