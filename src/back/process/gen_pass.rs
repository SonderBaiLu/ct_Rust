use crate::cli::genpass::GenPassOpts;
use rand::distributions::{Distribution, Uniform};
// 提取字符集常量，提升可维护性（避免视觉混淆的字符已剔除）
const UPPERCASE_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ"; // 去掉 I/O
const LOWERCASE_CHARS: &[u8] = b"abcdefghjklmnpqrstuvwxyz"; // 去掉 i/o
const DIGITS_CHARS: &[u8] = b"123456789"; // 去掉 0
const SPECIAL_CHARS: &[u8] = b"!@#$%^&*-_=+`~";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut chars = Vec::new();
    if opts.uppercase {
        chars.extend_from_slice(UPPERCASE_CHARS);
    }
    // 小写字母
    if opts.no_lowercase {
        chars.extend_from_slice(LOWERCASE_CHARS);
    }
    // 数字
    if opts.digits {
        chars.extend_from_slice(DIGITS_CHARS);
    }
    // 特殊字符
    if opts.special_chars {
        chars.extend_from_slice(SPECIAL_CHARS);
    }
    //校验字符集是否为空（避免无字符可生成密码）
    if chars.is_empty() {
        return Err(anyhow::anyhow!(
            "无法生成密码：未选择任何字符类型（大写/小写/数字/特殊字符）"
        ));
    }
    //校验密码长度是否合法
    if opts.length == 0 {
        return Err(anyhow::anyhow!("密码长度不能为 0"));
    }
    // 生成随机密码（使用更高效的 Uniform 分布，避免重复调用 choose）
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0..chars.len());
    let password: String = (0..opts.length)
        .map(|_| chars[dist.sample(&mut rng)] as char)
        .collect();

    // 5. 输出/返回密码（这里示例直接打印，你可根据需求改为返回 String）
    println!("生成的密码：{}", password);

    Ok(())
}
