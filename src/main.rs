use std::{env::args, fs, path::PathBuf};

fn append_to_path(p: PathBuf, s: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push(s);
    p.into()
}

fn main() {
    let mut output_path = home::home_dir().unwrap();
    output_path.push("argsbounce");
    let previous_runs = fs::read_dir(&output_path).unwrap();
    let new_run_id = previous_runs.count().to_string();
    output_path.push(new_run_id);

    fs::create_dir_all(&output_path).unwrap();

    for (index, arg) in args().enumerate() {
        fs::write(
            append_to_path(
                output_path.clone(),
                &("/".to_string() + &index.to_string() + ".txt"),
            ),
            arg,
        )
        .expect("Unable to write file");
    }
}
