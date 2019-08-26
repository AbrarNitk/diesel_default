# Encryption and Decryption

* Encryption and Decryption in action
* Initiate your secret key and use encode and decode functions.
* This features useful when we are using database related stuff and wanted to send id of table,
 So it is better to send encrypted_id rather than directly table row id.
* Secret key is used as global and sub_key is used as table wise.   
* It is same as django [encrypted_id](https://pypi.org/project/django-encrypted-id/).
* We can reuse the secret key as we are using in django to en-decrypt the id.

```rust
use crate::{
    prelude::{encode_ekey_util, decode_ekey_util, init_encrypt_conf}
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enc_test() {
        // We have to set secret key only single time based on that it will encrypt and decrypt key. 
        init_encrypt_conf("se(vh!38e21qca#9m7g0#5plq+a*z#imfjr10&iezsfmh6l)v(");
        let ekey = encode_ekey_util(5, "sub_key_foo").unwrap();
        let dkey = decode_ekey_util(&ekey, "sub_key_foo").unwrap();
        assert_eq!("51N8eu8UTVCCZiyLXQuARQ", ekey);
        assert_eq!(5, dkey);
    }
}

```