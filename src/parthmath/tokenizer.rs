pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>
}