use anyhow::Result;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=migrations");

    if let Ok(database_url) = dotenvy::var("DATABASE_URL") {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

            let m = Migrator::new(std::path::Path::new("./migrations"))
                .await
                .unwrap();
            m.run(&pool).await.unwrap();
        });
    }

    rosetta_build::config()
        .source("bg", "locales/bg.json")
        .source("cs", "locales/cs.json")
        .source("da", "locales/da.json")
        .source("de", "locales/de.json")
        .source("el", "locales/el.json")
        .source("en", "locales/en.json")
        .source("es", "locales/es.json")
        .source("fi", "locales/fi.json")
        .source("fr", "locales/fr.json")
        .source("hu", "locales/hu.json")
        .source("id", "locales/id.json")
        .source("it", "locales/it.json")
        .source("ja", "locales/ja.json")
        .source("ko", "locales/ko.json")
        .source("nl", "locales/nl.json")
        .source("no", "locales/no.json")
        .source("pl", "locales/pl.json")
        .source("ro", "locales/ro.json")
        .source("ru", "locales/ru.json")
        .source("sv", "locales/sv-SE.json")
        .source("tr", "locales/tr.json")
        .source("uk", "locales/uk.json")
        .source("zh", "locales/zh-CN.json")
        .fallback("en")
        .generate()?;

    Ok(())
}
