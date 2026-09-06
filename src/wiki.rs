use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use dirs;


fn get_sandbox_dir() -> PathBuf {
    let sandbox_dir = if let Ok(path) = env::var("FASTSAND_DATA_DIR") {
        PathBuf::from(path)
    } else {
        dirs::data_dir().expect(".").join("fastsand")
    };
    if !sandbox_dir.exists() {
        let _ = fs::create_dir_all(&sandbox_dir);
    }
    sandbox_dir
}

#[allow(dead_code)]
pub struct Wiki {
    category: String,
    name: String,
    args: HashMap<String, String>,
    dir: PathBuf,
}


impl Wiki {
    pub fn new(path: String) -> Self {
        let mut list = path.split("/");
        let fullname = list.next().expect("start").to_owned();
        let mut args = HashMap::new();
        loop {
            if let Some(k) = list.next() {
                if let Some(v) = list.next() {
                    args.insert(k.to_owned(), v.to_owned());
                    continue;
                }
            }
            break;
        }
        let category_ref;
        let name_ref;
        if let Some(divider) = &fullname.find(":") {
            let position = divider.to_owned();
            category_ref = &fullname[0..position];
            name_ref = &fullname[position+1..];
        } else {
            category_ref = "_default";
            name_ref = &fullname;
        }
        let category = category_ref.to_owned();
        let name = name_ref.to_owned();
        let sandbox_dir = get_sandbox_dir();
        let dir = sandbox_dir.join(&category).join(&name);
        Wiki { category, name, args, dir }
    }

    pub fn get_source(&self) -> String {
        if self.dir.exists() {
            fs::read_to_string(&self.dir).unwrap()
        } else {
            "404".to_owned()
        }
    }

    pub fn render(&self) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <title>处理结果</title>
</head>
<body>
    <p>文件位置：{}</p>
    <code>{}</code>
</body>
</html>
"#,
            self.dir.display(),
            self.get_source(),
        )
    }
}
