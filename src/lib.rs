
#[derive(Debug, Default, Clone)]
pub struct HostConfig {
    pub name: Option<String>,
    pub user: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
}

pub fn parse_config_file(path: &str) -> Vec<HostConfig> {
    let content = std::fs::read_to_string(path).unwrap();
    println!("Content: {:?}", content);
    let cfg = parse(&content);
    println!("Config: {:?}", cfg);
    cfg
}

pub fn parse(file: &str) -> Vec<HostConfig> {
    let mut res: Vec<HostConfig> = Vec::new();
    let mut cfg: HostConfig = HostConfig::default();
    for line in file.lines() {
        if let Some((k, v)) = parse_line(line.trim()) {
            match k.to_lowercase().as_str() {
                "host" => {
                    if let Some(_) = cfg.name {
                        res.push(cfg.clone())
                    } else {
                        cfg = HostConfig::default();
                        cfg.name = Some(v.to_string());
                    }
                },
                "user" => cfg.user = Some(v.to_string()),
                "hostname" => cfg.hostname = Some(v.to_string()),
                "port" => cfg.port = v.parse().ok(),
                "identityfile" => cfg.identity_file = Some(v.to_string()),
                _ => println!("K={:?} V={:?}", k, v)
            }
            println!("Config: {:?}", cfg);
        } else {
            continue
        }
    }
    res
}

fn parse_line(line: &str) -> Option<(&str, &str)> {
    let split_eq = split_line(line, "=");
    match split_eq {
        (None, Some(split)) => split_line(split, " ").0,
        (Some(kv), _) => Some(kv),
        _ => None
    }
}

fn split_line<'a>(line: &'a str, sep: &str) -> (Option<(&'a str, &'a str)>, Option<&'a str>) {
    let split: Vec<&str> = line.splitn(2, sep).collect();
    let trimmed: Vec<&str> = split.iter().map(|s| s.trim().trim_matches('"')).collect();
    match trimmed.len() {
        2 => (Some((trimmed[0], trimmed[1])), None),
        1 => (None, Some(trimmed[0])),
        _ => (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs::home_dir;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn local() {
        let mut f = home_dir().unwrap();
        f.push(".ssh");
        f.push("config");
        parse_config_file(f.to_str().unwrap());
    }
}