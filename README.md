# 使用Rust 编写的 命令行工具
## CLI 处理 csv
- 学习 clap crate 基本用法
- 构建基本的CLI 的结构i
- 使用 duckdb 查看 csv
- 学习csv crate 基本用法
- 学习 serde 基本用法
- 读取 csv 输出到 console
- 学习 serde-json 基本用法
- 将csv 转换成 json
- 重构代码
# RCLI
Rcli csv命令  -i [文件名称].csv  -o [文件名称].json -c -header -d '[生成的类型]'
#### 随机生成密码
问题： 是否需要： 大小写，长度，数字，特殊字符，强度检测
工具： rand crate


cargo add clap --features derive
