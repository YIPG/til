struct OwnedParsed {
    buf: Vec<u8>,
    repr: Parsed,
}

struct Owned {
    name: &'a str,
}

fn parse<'a>(input: &'a [u8]) -> Result<Parsed<'a>, _> {}
