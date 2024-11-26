use std::env;

const DEFAULT_TIMEOUT: u64 = 60000;
const DEFAULT_PIPE_NAME: &str = "cord-ipc";

#[derive(Debug)]
pub struct Args {
    pub pipe_name: String,
    pub client_id: String,
    pub timeout: u64,
}

impl Args {
    pub fn parse() -> crate::Result<Args> {
        let args: Vec<String> = env::args().collect();

        let mut pipe_name = None;
        let mut client_id = None;
        let mut timeout = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--pipe-name" | "-p" => {
                    if i + 1 < args.len() {
                        pipe_name = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err("Missing value for pipe-name".into());
                    }
                }
                "--client-id" | "-c" => {
                    if i + 1 < args.len() {
                        client_id = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err("Missing value for client-id".into());
                    }
                }
                "--timeout" | "-t" => {
                    if i + 1 < args.len() {
                        match args[i + 1].parse() {
                            Ok(t) if t > 0 => timeout = Some(t),
                            _ => return Err("Timeout must be a positive number".into()),
                        }
                        i += 2;
                    } else {
                        return Err("Missing value for timeout".into());
                    }
                }
                other => {
                    return Err(format!("Unknown argument: {}", other).into());
                }
            }
        }

        Ok(Args {
            pipe_name: pipe_name.unwrap_or_else(|| DEFAULT_PIPE_NAME.to_string()),
            client_id: client_id.ok_or("Missing client-id argument")?,
            timeout: timeout.unwrap_or(DEFAULT_TIMEOUT),
        })
    }
}
