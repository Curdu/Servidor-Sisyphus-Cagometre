use std::{env, sync::LazyLock};

use dotenvy::dotenv;

pub(crate) static SECRET_KEY : LazyLock<String> = LazyLock::new(||{
    dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY no esta definida a l'arxiu .env")
});

pub(crate) static DATABASE_URL : LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL no esta configurat")
});

pub(crate) static DATABASE_TYPE : LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    env::var("DATABASE_TYPE").expect("DATABASE_TYPE no esta configurat")
});