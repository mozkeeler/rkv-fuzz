#[macro_use]
extern crate afl;
extern crate rkv;

use rkv::{Rkv, StoreOptions};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    fuzz!(|data: &[u8]| {
        // First 8192 bytes are for the lock file.
        if data.len() < 8192 {
            return;
        }
        let (lock, db) = data.split_at(8192);
        let mut lock_file = match File::create("data.mdb") {
            Ok(lock_file) => lock_file,
            Err(_) => return,
        };
        match lock_file.write_all(lock) {
            Ok(_) => {}
            Err(_) => return,
        };
        let mut db_file = match File::create("data.mdb") {
            Ok(db_file) => db_file,
            Err(_) => return,
        };
        match db_file.write_all(db) {
            Ok(_) => {}
            Err(_) => return,
        };
        let env = {
            let mut builder = Rkv::environment_builder();
            builder.set_max_dbs(2);
            match Rkv::from_env(Path::new("."), builder) {
                Ok(env) => env,
                Err(_) => return,
            }
        };
        let store = match env.open_single("cert_storage", StoreOptions::create()) {
            Ok(store) => store,
            Err(_) => return,
        };
        let reader = match env.read() {
            Ok(reader) => reader,
            Err(_) => return,
        };
        match store.get(&reader, &[0]) {
            Ok(_) => {}
            Err(_) => {}
        };
    })
}
