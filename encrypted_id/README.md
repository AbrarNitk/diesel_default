# Encryption and Decryption

* Encryption and Decryption in action
* To make any struct encryptable, It must have an id field of type int necessary.
* To make any struct encryptable or decryptable, It must define sub_key to encrypt and decrypt key individually.
* Before using encryptable or decryptable, It must have initialize secret_key initially.
* Secret key length depends upon how much good encryption that you want (64 byte recommended).
* It is same as django [encrypted_id](https://pypi.org/project/django-encrypted-id/).
* We can reuse the secret key as we are using in django to en-decrypt the id.

```rust

use encrypted_id::{prelude::*};
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
        // We have to set secret key only single time based on that it will encrypt and decrypt key. 
        init_encrypt_conf("se(vh!38e21qca#9m7g0#7tyq+a*z#imfjr10&iezsfmh6l)v(");
        let e = EncyDemo{id: 5, name: "foo".to_string()};
        let ekey = e.ekey().unwrap();
        let dkey = e.dkey(&ekey).unwrap();
        assert_eq!("mZZLspleIzJqmKLa2Oio_g", ekey);
        assert_eq!(5, dkey);
    }
}

```