# 使用Rust 编写的 命令行工具
## CLI 处理 csv
- 学习 clap crate 基本用法
- 构建基本的CLI 的结构i
- 使用 duckdb 查看 csv
- 学习csv crate 基本用法
- 学习 Serde 基本用法
- 读取 csv 输出到 console
- 学习 serde-json 基本用法
- 将csv 转换成 json
- 重构代码
# RCLI
Rcli csv命令  -i [文件名称].csv  -o [文件名称].json -c -header -d '[生成的类型]'
#### 随机生成密码
问题： 是否需要： 大小写，长度，数字，特殊字符，强度检测
工具： rand crate
# 需要学习内容
1. 下列代码中的 .exists() 方法是用来检查文件是否存在的 如何使用？
2. 下列代码中的 .into() 方法是用来将字符串转换为 String 类型的 如何使用？
3. &'static str 是一个静态字符串 它的生命周期是整个程序运行时 它的内存是在程序加载时分配的 不会在运行时分配内存 也不会在运行时释放内存
~~~ rust
 fn verify_input_file(csv_file: &str) -> Result<String,&'static str> {
    if std::path::Path::new(csv_file).exists() {
        Ok(csv_file.into())
    } else {
        Err("文件不存在".into())
    }
    let records = reader
    .deserialize()
    .map(|record| record.unwrap())
    .collect::<Vec<Player>>();
~~~