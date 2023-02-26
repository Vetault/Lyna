use anyhow::Result;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=migrations");
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let pool = PgPoolOptions::new()
            .connect(&dotenvy::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let m = Migrator::new(std::path::Path::new("./migrations"))
            .await
            .unwrap();
        m.run(&pool).await.unwrap();
    });

    rosetta_build::config()
        .source("en", "locales/en.json")
        .source("fr", "locales/fr.json")
        .fallback("en")
        .generate()?;

    Ok(())
}
