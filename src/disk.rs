use std::collections::HashMap;
use tokio::process::Command;

pub struct DiskSpaceResult {
    pub used: u64,
    pub avail: u64,
    pub path: String,
    pub total: u64,
    pub percent: u8,
}

impl DiskSpaceResult {
    pub fn new(used: u64, avail: u64, path: String) -> Self {
        let total = used + avail;

        let percent = ((used as f64 / total as f64) * 100.0) as u8;

        Self {
            used,
            avail,
            path,
            total,
            percent,
        }
    }
}

pub async fn analyse_disk_space_at(
    paths: Vec<&str>,
) -> Result<Vec<DiskSpaceResult>, Box<dyn std::error::Error>> {
    let mut map: HashMap<&str, Option<DiskSpaceResult>> = HashMap::new();

    for path in paths {
        map.insert(path, None);
    }

    let output = Command::new("df").output();
    let output = output.await?;

    let std_out = String::from_utf8(output.stdout);

    if let Err(e) = std_out {
        return Err(Box::new(e));
    }

    let std_out = std_out.unwrap();
    let lines = std_out.split("\n");

    for line in lines {
        let parts: Vec<&str> = line.split(" ").filter(|part| part.len() > 1).collect();

        if parts.len() < 4 {
            continue;
        }

        let last = match parts.last() {
            None => {
                continue;
            }
            Some(value) => value,
        };

        if !map.contains_key(last) {
            continue;
        }

        let used: u64 = parts[2].parse().unwrap_or(0);
        let avail = parts[3].parse().unwrap_or(0);

        map.insert(
            last,
            Some(DiskSpaceResult::new(used, avail, last.to_string())),
        );
    }

    let mut results = vec![];

    for (_, res) in map {
        if let Some(res) = res {
            results.push(res);
        }
    }

    Ok(results)
}
