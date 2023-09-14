lazy_static::lazy_static! {
    pub static ref C:&'static str = include_str!("c.rs.liquid");
    pub static ref TYPESCRIPT:&'static str = include_str!("ts.rs.liquid");
}
