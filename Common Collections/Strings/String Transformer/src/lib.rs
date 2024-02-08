pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];
        for(data, command) in input{
            output.push(
                match command {
                    Command::Uppercase => data.to_uppercase(),
                    Command::Trim => data.trim().to_string(),
                    Command::Append(count) =>  format!("{}{}", data, "bar".repeat(count)),
                })
        }
        output
    }
}