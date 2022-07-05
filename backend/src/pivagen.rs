use messages_generator::Generator;
use std::collections::HashMap;
use std::fs::{read_dir, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::path::Path;

pub struct Piva {
    messages: Generator,
    additional: HashMap<String, Generator>, //<Keyword, Chain>
}

impl Piva {
    pub fn new() -> io::Result<Piva> {
        let messages = load_dataset("db/messages.txt")?;
        let additional = load_additional("db/addons").unwrap_or(HashMap::new());
        Ok(Piva {
            messages,
            additional,
        })
    }
    pub fn save_message(&mut self, mut message: String) -> io::Result<()> {
        if message.len() < 500
            && message.len() > 10
            && !(message.contains('/') || message.contains('[') || message.contains(']'))
        {
            self.messages.train(&message);
            let mut file = OpenOptions::new().append(true).open("db/messages.txt")?;
            message.push('\n');
            file.write_all(&message.as_bytes())?;
        }
        Ok(())
    }
    pub fn generate_answer(&self, message: &str) -> Option<String> {
        let message = message.to_lowercase();
        for (k, v) in self.additional.iter() {
            if message.contains(k) {
                return v.generate(20);
            }
        }
        self.messages.generate(20)
    }
}

fn load_dataset<T>(path: T) -> io::Result<Generator>
where
    T: AsRef<Path> + std::fmt::Debug,
{
    println!("Loading: {:?} ", &path);
    let mut generator = Generator::new();
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    for line in text.split('\n') {
        generator.train(line);
    }
    println!("OK");
    Ok(generator)
}
fn load_additional(path: &str) -> io::Result<HashMap<String, Generator>> {
    let mut chains = HashMap::new();
    for file in read_dir(path)? {
        if let Ok(file) = file {
            if file.file_type()?.is_file() {
                let keyword = {
                    let file = OpenOptions::new().read(true).open(file.path())?;
                    let mut reader = std::io::BufReader::new(file);
                    let mut key = String::new();
                    reader.read_line(&mut key)?;
                    key.pop();
                    println!("{}", key);
                    key
                };
                let chain = load_dataset(file.path())?;
                chains.insert(keyword, chain);
            }
        }
    }
    Ok(chains)
}
