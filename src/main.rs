use anyhow::Result;
mod task_2;
mod task_3;
mod task_4;
mod task_5;

fn main() -> Result<()> {
    // task_2::task_2().unwrap();
    // task_3::task_3();
    let (search, path) = task_4::task_4();
    let response = task_5::read_file(&path.to_str().unwrap());
    println!(
        "{}",
        response?
            .lines()
            .filter(|line| line.contains(&search))
            .count()
    );
    Ok(())
}
