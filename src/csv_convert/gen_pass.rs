use crate::opts::GenPassOpts;

fn  process_genpass(opts: &GenPassOpts) -> anyhow::Result<()>{
    let mut rng = rand::rng(); //生成随机数字
    let mut password = String::new();
    let mut chars = Vec::new();
    if opts.uppercase { // 去掉了 "I" "i" "O" "0" 避免了视觉上的误认
        chars.extend_from_slice(b"ABCDEFGHJKLMNOPQRSTUVWXYZ");
    }
    if opts.lowercase {
        chars.extend_from_slice(b"abcdefghjklmnpqrstuvwxyz")
    }
    if opts.numbers{
        chars.extend_from_slice(b"123456789");
    }
    if opts.symbol {
        chars.extend_from_slice(b"!@#$%^&*-_=+`~");
    }
    todo!()
}




























