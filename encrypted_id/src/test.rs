use crate::{
    prelude::{encode_ekey_util, decode_ekey_util, init_encrypt_conf}
};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<String> {
    let f1: BufReader<File> =
        BufReader::new(File::open(path).expect(&format!("Not able to read file : {}", path)));
    let mut lines = vec![];
    for it in f1.lines() {
        lines.push(it.unwrap())
    }
    lines
}

fn read_secret_key() -> String {
    let secret_key = read_file("./secret_key.txt");
    secret_key
        .get(0)
        .map(|x| x.to_string())
        .expect("Could not found secret key")
}

fn read_tests() -> Vec<(u64, String)> {
    read_file("./test.txt")
        .into_iter()
        .map(|x| {
            let t = x.split(",").collect::<Vec<&str>>();
            (t[0].parse::<u64>().unwrap(), t[1].trim().to_string())
        })
        .collect::<Vec<(u64, String)>>()
}

#[test]
fn encrypted_id() {
    let secret_key: &str = &read_secret_key();
    init_encrypt_conf(secret_key);
    for (i, expected) in read_tests() {
        match decode_ekey_util(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };
        match encode_ekey_util(i, &format! {"{}", i}) {
            Ok(encoded) => assert_eq!(encoded, expected),
            Err(_) => assert!(false),
        };
    }
}

#[test]
fn enc_test() {
    init_encrypt_conf("se(vh!38e21qca#9m7g0#5plq+a*z#imfjr10&iezsfmh6l)v(");
    let ekey = encode_ekey_util(5, "sub_key_foo").unwrap();
    let dkey = decode_ekey_util(&ekey, "sub_key_foo").unwrap();
    assert_eq!("51N8eu8UTVCCZiyLXQuARQ", ekey);
    assert_eq!(5, dkey);
}
