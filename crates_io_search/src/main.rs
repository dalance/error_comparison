use crates_io_api::SyncClient;

fn main() {
    let client = SyncClient::new();
    let all = client.all_crates(Some(String::from("error")));
    let mut result = Vec::new();
    if let Ok(all) = all {
        for c in all {
            let name = c.name;
            let created = c.created_at;
            let updated = c.updated_at;
            if let Some(d) = c.description {
                if d.to_lowercase().contains("error") {
                    let rev = client.crate_reverse_dependencies(&name);
                    if let Ok(rev) = rev {
                        let total = rev.meta.total;
                        if total != 0 {
                            result.push((total, name, created, updated));
                        }
                    }
                }
            }
        }
    }

    result.sort_by_key(|x| x.0);
    result.reverse();
    println!("|crate|dependents|created_at|updated_at|");
    println!("|-----|----------|----------|----------|");
    for (total, name, created, updated) in result {
        println!(
            "|{}|{}|{}|{}|",
            name,
            total,
            created.format("%Y-%m-%d"),
            updated.format("%Y-%m-%d")
        );
    }
}
