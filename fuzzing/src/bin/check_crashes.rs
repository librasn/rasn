fn main() -> Result<(), Box<dyn std::error::Error>> {
    for file in std::fs::read_dir("out/default/crashes/")?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
    {
        let case = file.file_stem().unwrap().to_str().unwrap().to_owned();
        println!("Testing Crash case: `{}`", case);
        fuzz::fuzz(&std::fs::read(file)?);
        println!("Testing Crash case: `{}` successful", case);
    }

    println!("No valid crash cases found.");

    Ok(())
}
