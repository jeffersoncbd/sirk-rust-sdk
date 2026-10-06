use crate::{Error, Sirk, transport::AgentRunResponse};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

impl Sirk {
    pub fn agent(&self, agent: &str, input: &str) -> Result<String, Error> {
        let mut input = input.to_owned();
        let mut conversation_id = None;
        loop {
            match self.transport.run_agent(
                &self.directory,
                &self.flow_id,
                agent,
                conversation_id.as_deref(),
                &input,
            )? {
                AgentRunResponse::Ask {
                    question,
                    conversation_id: id,
                } => {
                    input = ask_user(&question)?;
                    conversation_id = Some(id);
                }
                AgentRunResponse::Result(result) => return Ok(result),
            }
        }
    }
}

fn ask_user(question: &str) -> Result<String, Error> {
    let terminal = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .map_err(|error| Error::Tool(format!("ASK requires an interactive terminal: {error}")))?;
    let mut reader = BufReader::new(
        terminal
            .try_clone()
            .map_err(|error| Error::Tool(format!("could not access terminal: {error}")))?,
    );
    let mut writer = terminal;
    read_answer(&mut reader, &mut writer, question)
}

fn read_answer(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    question: &str,
) -> Result<String, Error> {
    write!(writer, "{question}\n> ")
        .map_err(|error| Error::Tool(format!("could not write agent question: {error}")))?;
    writer
        .flush()
        .map_err(|error| Error::Tool(format!("could not flush agent question: {error}")))?;
    let mut answer = String::new();
    if reader
        .read_line(&mut answer)
        .map_err(|error| Error::Tool(format!("could not read agent answer: {error}")))?
        == 0
    {
        return Err(Error::Tool("agent answer input is closed".into()));
    }
    let answer = answer.trim_end_matches(['\r', '\n']);
    if answer == "/cancel" {
        return Err(Error::Tool("agent question cancelled".into()));
    }
    Ok(answer.to_owned())
}

#[cfg(test)]
mod tests {
    use super::read_answer;
    use std::io::Cursor;

    #[test]
    fn reads_an_agent_answer_after_printing_its_question() {
        let mut reader = Cursor::new(b"Ada\r\n");
        let mut writer = Vec::new();

        let answer = read_answer(&mut reader, &mut writer, "What is your name?").unwrap();

        assert_eq!(answer, "Ada");
        assert_eq!(writer, b"What is your name?\n> ");
    }

    #[test]
    fn cancels_an_agent_question() {
        let mut reader = Cursor::new(b"/cancel\n");
        let mut writer = Vec::new();

        let error = read_answer(&mut reader, &mut writer, "Continue?").unwrap_err();

        assert_eq!(
            error.to_string(),
            "S.I.R.K. host tool failed: agent question cancelled"
        );
    }
}
