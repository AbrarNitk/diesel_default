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
        init_encrypt_conf("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h6l");
        let ekey = encode_ekey_util(5, "sub_key_foo").unwrap();
        let dkey = decode_ekey_util(&ekey, "sub_key_foo").unwrap();
        assert_eq!("E86VGQhfxb_9rxSfjnBqKg", ekey);
        assert_eq!(5, dkey);
    }
}

```

## Using with struct 
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
        init_encrypt_conf("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h6l");
        let e = EncyDemo{id: 5, name: "foo".to_string()};
        let ekey = e.ekey().unwrap();
        let dkey = e.dkey(&ekey).unwrap();
        assert_eq!("E86VGQhfxb_9rxSfjnBqKg", ekey);
        assert_eq!(5, dkey);
    }
}

```