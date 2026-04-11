use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    //命令生成器：每个参数的简选项名称必须唯一
    #[arg(short, long, default_value_t = 16)]
    pub length: usize, // 密码长度
    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool, // 是否包含大写字母
    #[arg(short, long, default_value_t = true)]
    pub no_lowercase: bool, // 是否包含小写字母
    #[arg(short, long, default_value_t = true)]
    pub digits: bool, // 是否包含数字
    #[arg(short, long, default_value_t = true)]
    pub special_chars: bool, // 是否包含特殊字符
}
