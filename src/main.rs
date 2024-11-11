use std::env;
use std::process;
use rand::{distributions::Alphanumeric, Rng};

fn generate_random_name() -> String {
    let name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("container_{}", name)
}

#[derive(Debug)]
struct Docker {
    name: String,
    config: Config,
}

impl Docker {
    fn new(config: Config) -> Docker {
        let name = generate_random_name();
        Docker { name, config }
    }

    fn pull_docker_image(&self) -> Result<(), String> {
        let output = process::Command::new("docker")
            .arg("pull")
            .arg(&self.config.docker_image)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "Docker pull failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn run_docker_image_as_daemon(&self) -> Result<(), String> {
        let output = process::Command::new("docker")
            .arg("run")
            .arg("--name")
            .arg(&self.name)
            .arg("-d")
            .arg("-ti")
            .arg("--rm")
            //.arg("--entrypoint")
            //.arg("sh")
            .arg(&self.config.docker_image)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "Docker run failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}

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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: {} <docker_image>", args[0]);
        process::exit(1);
    });

    println!("Docker image: {}", config.docker_image);
    let docker = Docker::new(config);

    if let Err(e) = docker.pull_docker_image() {
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
        let docker = Docker::new(config);
        let result = docker.pull_docker_image();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pull_docker_image_failure() {
        let config = Config {
            docker_image: String::from("non_existent_image"),
        };
        let docker = Docker::new(config);
        let result = docker.pull_docker_image();
        assert!(result.is_err());
    }

    #[test]
    fn test_run_docker_image_as_daemon_success() {
        let config = Config {
            docker_image: String::from("alpine"),
        };
        let docker = Docker::new(config);
        let _ = docker.pull_docker_image();
        let result = docker.run_docker_image_as_daemon();
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_docker_image_as_daemon_failure() {
        let config = Config {
            docker_image: String::from("non_existent_image"),
        };
        let docker = Docker::new(config);
        let result = docker.run_docker_image_as_daemon();
        assert!(result.is_err());
    }

    #[test]
    fn test_docker_new() {
        let config = Config {
            docker_image: String::from("hello-world"),
        };
        let docker = Docker::new(config);
        assert_eq!(docker.config.docker_image, "hello-world");
        assert!(docker.name.starts_with("container_"));
    }


}
