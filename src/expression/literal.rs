pub enum LiteralValue<'a> {
    True,
    False,
    None,
    Number(&'a f64),
    String(&'a str),
}

pub struct Literal<'a> {
    pub value: LiteralValue<'a>,
}
