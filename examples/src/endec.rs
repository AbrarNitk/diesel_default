use crate::Result;
use encrypted_id::prelude::*;

#[derive(Debug, Default, Encrypted, Decrypted)]
#[encdec_opts(opts(sub_key = "enky_demo_sub_key"))]
pub struct EncyDemo {
    pub id: u64,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enc_test() {
        init_encrypt_conf("se(vh!38e21qca#9m7g0#7tyq+a*z#imfjr10&iezsfmh6l)v(");
        let e = EncyDemo {
            id: 5,
            name: "foo".to_string(),
        };
        let ekey = e.ekey().unwrap();
        let dkey = e.dkey(&ekey).unwrap();
        assert_eq!("mZZLspleIzJqmKLa2Oio_g", ekey);
        assert_eq!(5, dkey);
    }
}
