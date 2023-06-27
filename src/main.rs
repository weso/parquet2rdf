use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    use parquet2::read::read_metadata;
    let mut reader = std::fs::File::open(path)?;
    let metadata = read_metadata(&mut reader)?;

    println!("{:#?}", metadata);
    Ok(())
}
