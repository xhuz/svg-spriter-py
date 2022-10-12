use std::{collections::HashMap, fs, path::Path};

mod optimize;
mod spriter;

fn main() {
    // let raw = read_to_string("raw/0.svg").unwrap();
    // let result = optimize::optimize(&raw);
    // spriter::process(vec![result], "12.svg");

    let mut svgs = vec![];

    if let Ok(dir) = fs::read_dir("raw") {
        dir.filter_map(|item| item.ok()).for_each(|item| {
            let c = fs::read_to_string(Path::new(&format!(
                "raw/{}",
                item.file_name().to_str().unwrap()
            )))
            .unwrap();
            svgs.push(c)
        })
    }
    let mut d: Vec<HashMap<String, String>> = vec![];

    for i in 0..=10 {
        let mut map: HashMap<String, String> = HashMap::new();

        let a = &svgs[i];

        map.insert("id".to_string(), "".to_string());
        map.insert("content".to_string(), optimize::optimize(a));
        d.push(map);
    }

    // let c = fs::read_to_string("raw/0.svg").unwrap();

    // let c = optimize::optimize(&c);

    // fs::write("a1.svg", c.clone()).unwrap();

    // let mut map = HashMap::new();
    // map.insert("id".to_string(), "".to_string());
    // map.insert("content".to_string(), c);

    spriter::process(d, "a2.svg");
}
