extern crate arche;
extern crate log4rs;
#[macro_use]
extern crate log;

fn main() {
    let run = || {
        log4rs::init_file("log4rs.yml", Default::default())?;
        arche::console::run()
    };
    if let Err(e) = run() {        
        error!("{:?}", e);
    }
}
