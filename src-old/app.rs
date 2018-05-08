use super::orm::{self, Connection as Db};
use super::queue::{self, Provider as QueueProvider};
use super::result::{Error, Result};
use super::{cache, env, graphql, i18n, jwt, router, security};

pub fn server() -> Result<()> {
    // worker
    thread::spawn(|| loop {
        let worker = || -> Result<()> {
            let name = sys_info::hostname()?;
            log::info!("Starting worker thread {}", name);
            let etc = parse_config()?;
            queue::new(etc.queue.url.clone(), etc.queue.name.clone()).consume(
                queue::Consumer::new(
                    name,
                    etc.env()?,
                    orm::new(etc.database.clone())?,
                    security::Encryptor::new(etc.secret_key()?.as_slice())?,
                ),
            )?;
            Ok(())
        };
        match worker() {
            Ok(_) => log::info!("exiting worker"),
            Err(e) => log::error!("{:?}", e),
        }
        thread::sleep(Duration::from_secs(10));
    });
    // http
    let etc = parse_config()?;
    let cfg = rocket::config::Config::build(etc.env()?)
        .address("localhost")
        .workers(etc.workers)
        .secret_key(etc.secret_key.clone())
        .port(etc.http.port)
        .limits(etc.http.limits())
        .extra(
            "template_dir",
            match Path::new("themes")
                .join(&etc.http.theme)
                .join("views")
                .to_str()
            {
                Some(v) => v,
                None => "views",
            },
        )
        .finalize()?;
    let dbp = orm::new(etc.database.clone())?;
    let mut app = rocket::custom(cfg, false)
        .manage(dbp.clone())
        .manage(graphql::schema::Schema::new(
            graphql::query::Query {},
            graphql::mutation::Mutation {},
        ))
        .manage(cache::new(&etc.cache.url, etc.cache.namespace.clone())?)
        .manage(security::Encryptor::new(etc.secret_key()?.as_slice())?)
        .manage(queue::new(etc.queue.url.clone(), etc.queue.name.clone()))
        .manage()
        .manage(etc.clone());

    if !etc.is_prod() {
        app = app.mount("/", routes!(router::get_assets))
    }

    for (pt, rt) in router::routes() {
        app = app.mount(pt, rt);
    }

    app.attach(Template::fairing())
        .attach(etc.http.cors())
        .catch(router::catchers())
        .launch();

    Ok(())
}

pub fn generate_config() -> Result<()> {}

pub fn generate_nginx() -> Result<()> {}

pub fn db_seed() -> Result<()> {}

pub fn db_dump() -> Result<()> {
    // TODO
    Ok(())
}

pub fn db_restore(_name: &String) -> Result<()> {
    // TODO
    Ok(())
}

//-----------------------------------------------------------------------------
