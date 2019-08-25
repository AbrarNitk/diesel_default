use crate::Result;

pub trait Encrypted {
    fn ekey(&self) -> Result<String>;
}

pub trait Decrypted {
    fn dkey(&self, ekey: &str) -> Result<u64>;
}

#[derive(Debug, Default, Encrypted, Decrypted)]
#[encdec_opts(opts(sub_key = "endemo"))]
pub struct EncyDemo {
    pub id: u64,
    pub name: String,
}

pub fn encode_ekey_util(id: u64, sub_key: &str) -> Result<String> {
    Ok("".to_string())
}

pub fn decode_ekey_util(ekey: &str, sub_key: &str) -> Result<u64> {
    Ok(5)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enc_test() {
        let e = EncyDemo::default();
        //println!("{}", e.ekey());
        println!("{:?}", e.dkey("bakchodi"));
    }
}
