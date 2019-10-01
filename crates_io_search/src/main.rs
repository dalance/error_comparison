use crates_io_api::SyncClient;

fn main() {
    let client = SyncClient::new();
    let all = client.all_crates(Some(String::from("error")));
    let mut result = Vec::new();
    if let Ok(all) = all {
        for c in all {
            let name = c.name;
            let update = c.updated_at;
            if let Some(d) = c.description {
                if d.to_lowercase().contains("error") {
                    let rev = client.crate_reverse_dependencies(&name);
                    if let Ok(rev) = rev {
                        let total = rev.meta.total;
                        if total != 0 {
                            result.push((total, name, update));
                        }
                    }
                }
            }
        }
    }

    result.sort_by_key(|x| x.0);
    result.reverse();
    println!("|crate|dependents|updated_at|");
    println!("|-----|----------|----------|");
    for (total, name, update) in result {
        println!("|{}|{}|{}|", name, total, update.format("%Y-%m-%d"));
    }
}
