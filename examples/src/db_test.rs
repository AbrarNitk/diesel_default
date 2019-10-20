use crate::tables::users_skill::{self, dsl};

use crate::Result;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct ISkill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub allocation_logic: String,
}

#[derive(Default, DSave, Insertable, Debug, Clone)]
#[table_name = "users_skill"]
#[save_opts(opts(output_type = "ISkill", dsl_name = "users_skill"))]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub allocation_logic: String,
}

impl Skill {
    fn new(name: &str, desc: &str, all: &str) -> Self {
        Self {
            name: name.to_string(),
            description: desc.to_string(),
            allocation_logic: all.to_string(),
        }
    }
}

impl ISkill {
    fn new(name: &str, desc: &str, all: &str) -> Self {
        Self {
            id: 1,
            name: name.to_string(),
            description: desc.to_string(),
            allocation_logic: all.to_string(),
        }
    }
}

fn db_test() {
    let database_url = "postgres://root@127.0.0.1/acko";
    let _query = "select * from users_skill";
    let conn: PgConnection = PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));

     let r: Result<ISkill> = Skill::new("abcd", "desc", "dsadfasdf").save(&conn);
     println!("{:?}", r);
}

//trait DSave {
//    type Output;
//    //fn save(&self, conn: &PgConnection) -> Result<Output>;
//    fn save<Conn>(&self, conn: &Conn) -> Result<Self::Output>
//        where Conn: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager>;
//}

trait DSave {
    type Output;
    fn save(&self, conn: &PgConnection) -> Result<Self::Output>;
    fn save_vec(values: &[Self], conn: &PgConnection) -> Result<usize>
    where
        Self: std::marker::Sized;
}

#[cfg(test)]
mod tests {
    use super::db_test;

    #[test]
    fn test_select() {
        db_test()
    }
}
