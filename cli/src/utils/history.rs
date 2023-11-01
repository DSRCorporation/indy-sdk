use linefeed::{Terminal, Interface};

use crate::utils::environment::EnvironmentUtils;
use crate::utils::file::{read_lines_from_file, write_file};


const HISTORY_SIZE: usize = 100;
const SECRET_DATA: [&str; 2] = [" seed=", " key="];

pub fn load<T>(reader: &mut Interface<T>) -> Result<(), String> where T: Terminal {
    reader.set_history_size(HISTORY_SIZE);

    let path = EnvironmentUtils::history_file_path();

    for line in read_lines_from_file(path)? {
        if let Ok(line) = line {
            reader.add_history(line)
        }
    }
    Ok(())
}

pub fn persist<T>(reader: &Interface<T>) -> Result<(), String> where T: Terminal {
    let content =
        reader
            .lock_writer_erase()
            .map_err(|err| format!("{}", err))?
            .history()
            .filter(|record|
                !SECRET_DATA.iter().any(|secret_word| record.contains(secret_word))
            )
            .collect::<Vec<&str>>()
            .join("\n");

    let path = EnvironmentUtils::history_file_path();
    write_file(&path, &content)
}
