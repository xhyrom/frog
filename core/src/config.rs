use std::{fs, io::{Error, ErrorKind}, collections::HashMap};

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub commands: Vec<String>,
}

#[derive(Debug)]
pub struct Config {
    pub variables: HashMap<String, String>,
    pub tasks: Vec<Task>,
}

impl Config {
    pub fn serialize(self) -> String {
        let mut config = String::new();

        for variable in self.variables {
            config.push_str(&format!("declare {} = {}\n", variable.0, variable.1));
        }

        for task in self.tasks {
            config.push_str(
                &format!(
                    "\ntask {} {{\n{}\n}}\n",
                    task.name,
                    task.commands.iter().map(|x| "  ".to_owned() + x).collect::<Vec<String>>().join("\n")
                )
            );
        }

        config
    }
}

pub fn parse_config(content: String) -> Config {
    let mut variables = HashMap::new();
    let mut tasks = Vec::new();

    let mut task_name = String::new();
    let mut task_content = Vec::new();
    let mut task = false;

    for line in content.lines() {
        if line.starts_with("#") || line.starts_with("//") {
            continue;
        } else if line.starts_with("declare") && line.contains("=") {
            let temp_collected_variable = line.split("=").collect::<Vec<&str>>();

            variables.insert(
                temp_collected_variable[0].to_string().replace("declare", "").replace(" ", ""),
                cut_quotes(temp_collected_variable[1].trim().to_string())
            );
        } else if line.contains("task") && line.ends_with("{") {
            task_name = line.replace("task ", "").replace("{", "").trim().to_string();
            task = true;
        } else if task && line == "}" {
            task = false;
            tasks.push(Task {
                name: (&task_name).to_string(),
                commands: task_content,
            });
            task_content = Vec::new();
        } else if task {
            let mut line_copy = line.to_string();

            for var in variables.iter() {
                if line_copy.contains(var.0) {
                    line_copy = line_copy.replace((format!("{{{}}}", &var.0)).as_str(), var.1);
                }
            }

            task_content.push(
                format!("{}\n", line_copy.trim())
            );
        }
    }

    drop(task_name);
    drop(task_content);
    drop(task);

    return Config {
        variables,
        tasks,
    };
}

pub fn find(path: String) -> std::io::Result<Config> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_string = path.to_str().unwrap().to_string();

        if path_string.contains("uwu.frog") {
            let content = fs::read_to_string(path)?;
            drop(path_string);

            return Ok(parse_config(content));
        }
    }

    Err(Error::new(ErrorKind::NotFound, "uwu.frog not found"))
}

fn cut_quotes(string: String) -> String {
    return if string.starts_with("\"") && string.ends_with("\"") { string[1..string.len() - 1].to_string() } else { string };
}