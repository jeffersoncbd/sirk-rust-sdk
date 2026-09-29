use super::Tools;
use crate::Error;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

impl Tools<'_> {
    pub fn await_confirm(&self) -> Result<(), Error> {
        let terminal = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .map_err(|error| {
                Error::Tool(format!("AWAIT requires an interactive terminal: {error}"))
            })?;
        let mut reader = BufReader::new(
            terminal
                .try_clone()
                .map_err(|error| Error::Tool(format!("could not access terminal: {error}")))?,
        );
        let mut writer = terminal;
        write!(
            writer,
            "Press Enter to continue, or type /cancel to abort: "
        )
        .map_err(|error| Error::Tool(format!("could not write confirmation prompt: {error}")))?;
        writer.flush().map_err(|error| {
            Error::Tool(format!("could not flush confirmation prompt: {error}"))
        })?;
        let mut answer = String::new();
        if reader
            .read_line(&mut answer)
            .map_err(|error| Error::Tool(format!("could not read confirmation: {error}")))?
            == 0
        {
            return Err(Error::Tool("confirmation input is closed".into()));
        }
        if answer.trim_end_matches(['\r', '\n']) == "/cancel" {
            return Err(Error::Tool("confirmation cancelled".into()));
        }
        Ok(())
    }
}
