// 宏
#[allow(unused)]
#[derive(Debug, Builder)]
struct Builder {
    // setter(into) 表示任何传进来的值都会调用他的into方法
    #[builder(setter(into))]
    name: String,
    #[builder(default = "42")]
    age: u8,
    #[builder(default = "VEC![]"),setter(each(name = "skill", into))]
    skills: Vec<String>,
}
fn main() -> Result<()>{
    let user =User::builder()
        .name("Jason")
        .skill("Rust")
        .skill("C++")
        .build()?;
    println!("{:#?}", user);
    Ok(())
}

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}