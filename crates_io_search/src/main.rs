use chrono::Utc;
use crates_io_api::SyncClient;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

static BLACKLIST: &'static [&'static str] = &[
    "glium",
    "console_error_panic_hook",
    "textwrap",
    "abscissa",
    "syntex_errors",
    "amethyst_error",
    "tk-listen",
    "lark-error",
];

fn main() {
    let client = SyncClient::new();
    let all = client.all_crates(Some(String::from("error")));
    let mut result = Vec::new();
    if let Ok(all) = all {
        for c in all {
            let name = c.name;
            let created = c.created_at;
            let updated = c.updated_at;
            if BLACKLIST.contains(&name.as_str()) {
                continue;
            }
            if let Some(d) = c.description {
                if d.to_lowercase().contains("error") {
                    let rev = client.crate_reverse_dependencies(&name);
                    if let Ok(rev) = rev {
                        let total = rev.meta.total;
                        if total >= 10 {
                            result.push((total, name, created, updated));
                        }
                    }
                }
            }
        }
    }

    result.sort_by_key(|x| x.0);
    result.reverse();

    let date = Utc::today();

    let file = format!("{}/results.toml", env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)
        .unwrap();

    for (dependents, name, created, updated) in result {
        let _ = writeln!(file, "[{}.{}]", name, date.format("%Y-%m-%d"));
        let _ = writeln!(file, "dependents = {}", dependents);
        let _ = writeln!(file, "created = {}", created.format("%Y-%m-%d"));
        let _ = writeln!(file, "updated = {}", updated.format("%Y-%m-%d"));
    }

    //println!("|crate|dependents|created_at|updated_at|");
    //println!("|-----|----------|----------|----------|");
    //for (dependents, name, created, updated) in result {
    //    println!(
    //        "|{}|{}|{}|{}|",
    //        name,
    //        dependents,
    //        created.format("%Y-%m-%d"),
    //        updated.format("%Y-%m-%d")
    //    );
    //}
}
