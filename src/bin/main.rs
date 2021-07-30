use std::{time, thread, sync::mpsc};
use recollection::{RecolError, RecolResult,
    data::{Graph, Node, Edge},
    fs::{cat, copy, rm, touch, mkdir}
};

fn main() -> RecolResult<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for r in &["hello", "there", "baby", "what", "de", "heck"] {
            tx.send(r).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    for r in rx {
        println!("{}", r);
    }
    Ok(())
}

fn tests() -> RecolResult<()> {
    let r = cat("./Cargo.toml")?;
    let fi = touch("./test.toml")?;
    let td = mkdir("./testdir")?;
    // println!("{}", r);
    Ok(())
}
