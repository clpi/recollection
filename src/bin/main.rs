use recollection::{RecolError, RecolResult,
    data::{Graph, Node, Edge},
    fs::{cat, copy, rm, touch, mkdir}
};

fn main() -> RecolResult<()> {
    let r = cat("./Cargo.toml")?;
    let fi = touch("./test.toml")?;
    let td = mkdir("./testdir")?;

    println!("{}", r);
    Ok(())
}
