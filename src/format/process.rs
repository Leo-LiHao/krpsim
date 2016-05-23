use super::ressource::Ressource;

pub struct Process {
    pub name: String,
    pub cycle: u64,
    pub input: Vec<Ressource>,
    pub output: Vec<Ressource>
}

impl Process {
    pub fn new(
        name: &str,
        cycle: u64,
        input: Vec<Ressource>,
        output: Vec<Ressource>
        ) -> Self {
        Process {
            name: name.to_string(),
            cycle: cycle,
            input: input,
            output: output
        }
    }
}
