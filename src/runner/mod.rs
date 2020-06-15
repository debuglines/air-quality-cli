use crate::runner::runner::RunnerErrorResult;

use std::path::PathBuf;

mod bluetooth;
mod parser;
mod python_dependency;
pub mod runner;

pub fn start_data_generator(data_dir_path: PathBuf, serial_number: u32) -> RunnerErrorResult<()> {
    Ok(run(data_dir_path, serial_number)?)
}

pub fn run(data_dir_path: PathBuf, serial_number: u32) -> RunnerErrorResult<()> {
    let runner = runner::Runner::new(data_dir_path, serial_number)?;
    runner.run()?;
    Ok(())
}
