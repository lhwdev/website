use sea_orm::{Database, ConnectionTrait, Schema};

// Initalizes entities.
#[async_std::main]
async fn main() {
    //  Setting `DATABASE_URL` environment variable
    let key = "DATABASE_URL";
    if std::env::var(key).is_err() {
        // Getting the database URL from Rocket.toml if it's not set
        let figment = rocket::Config::figment();
        let database_url: String = figment
            .extract_inner("databases.sea_orm.url")
            .expect("Cannot find Database URL in Rocket.toml");
        std::env::set_var(key, database_url);
    }

    let database_url = format!("{}?mode=rwc", std::env::var(key).unwrap());
    let db = Database::connect(database_url).await.unwrap();
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    // actually creates table

    let statements = vec![
        builder.build(&schema.create_table_from_entity(entity::post::Entity)),
        builder.build(&schema.create_table_from_entity(entity::user::Entity)),
    ];

    for statement in statements {
        db.execute(statement).await.unwrap();
    }
}
