use std::env;
use std::process;

#[derive(Debug)]
struct Config {
    docker_image: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let docker_image = args[1].clone();
        Ok(Config { docker_image })
    }

    fn pull_docker_image(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output = process::Command::new("docker")
            .arg("pull")
            .arg(&self.docker_image)
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Failed to pull docker image: {}",
                String::from_utf8_lossy(&output.stderr)
            ).into());
        }

        println!("Successfully pulled docker image: {}", self.docker_image);
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: {} <docker_image>", args[0]);
        process::exit(1);
    });

    println!("Docker image: {}", config.docker_image);
    if let Err(e) = config.pull_docker_image() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_with_valid_args() {
        let args = vec![
            String::from("program_name"),
            String::from("valid_docker_image"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.docker_image, "valid_docker_image");
    }

    #[test]
    fn test_config_new_with_no_args() {
        let args: Vec<String> = vec![String::from("program_name")];
        let config = Config::new(&args);
        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "not enough arguments");
    }

    #[test]
    fn test_pull_docker_image_success() {
        let config = Config {
            docker_image: String::from("hello-world"),
        };
        let result = config.pull_docker_image();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pull_docker_image_failure() {
        let config = Config {
            docker_image: String::from("non_existent_image"),
        };
        let result = config.pull_docker_image();
        assert!(result.is_err());
    }
}
