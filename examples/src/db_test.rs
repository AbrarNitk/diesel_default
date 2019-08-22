use crate::tables::{
    users_skill::{dsl, self}
};

use diesel::{connection::{Connection}, prelude::*};
use crate::Result;
use std::process::Output;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "users_skill"]
pub struct ISkill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub allocation_logic: String,
}

#[diesel_default(output_type = "ISkill", dsl_name = "users_skill")]
#[derive(Insertable, Default, Debug, Clone)]
#[table_name = "users_skill"]
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
            allocation_logic: all.to_string()
        }
    }
}

impl ISkill {
    fn new(name: &str, desc: &str, all: &str) -> Self {
        Self {
            id: 1,
            name: name.to_string(),
            description: desc.to_string(),
            allocation_logic: all.to_string()
        }
    }
}

fn db_test(){

    let database_url = "postgres://root@127.0.0.1/acko";
    let _query = "select * from users_skill";
    let conn: PgConnection = PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));

     let r: Result<ISkill> = Skill::new("abcd", "desc", "dsadfasdf").save(&conn);
     println!("{:?}", r);

//    let r: Result<ISkill> = diesel::insert_into(dsl::users_skill)
//        .values(Skill::new("abcd", "desc", "dsadfasdf"))
//        .get_result(&conn)
//        .map_err(|e|e.into());
//    println!("{:?}", r);

//    let result: Vec<ISkill> = dsl::users_skill.filter(dsl::id.gt(20))
//        .load::<ISkill>(&conn)
//        .expect("Error loading skills");
//    for x in result.iter() {
//        println!("{:?}", x);
//    }
}

trait DieselSave {
    type Output;
    fn save(&self, conn: &PgConnection) -> Result<Output>;
}

#[derive(MyTrait)]
#[my_crate(lorem(dolor="Hello"))]
pub struct Consumer{}


#[cfg(test)]
mod tests {
    use super::db_test;

    #[test]
    fn test_select() {
        db_test()
    }
}