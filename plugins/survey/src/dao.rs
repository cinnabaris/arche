use nut::orm::postgresql;

pub trait Dao {
    fn get();
}

impl<'a> Dao for postgresql::Dao<'a> {
    fn get() {}
}
