pub struct Wiki {
    path: String,
}

impl Wiki {
    pub fn new(path: String) -> Self {
        Wiki { path }
    }
    pub fn render(&self) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html>
<head><title>处理结果</title></head>
<body>
    <h1>你输入的路径是: {}</h1>
    <p>这个页面是由 Rust 生成的。</p>
</body>
</html>
"#,
            self.path,
        )
    }
}
