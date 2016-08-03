#[macro_use]
extern crate nom;
extern crate copperline;
extern crate stockfighter;

use nom::{IResult, space, alphanumeric};
use copperline::Copperline;
use stockfighter::Stockfighter;

named!(quote<&[u8], (&str,&str)>,
    chain!(
        tag!("quote") ~
        space? ~
        venue: map_res!(alphanumeric, std::str::from_utf8) ~
        space? ~
        stock: map_res!(alphanumeric, std::str::from_utf8) ,

        || (venue, stock)
    )
);


fn main() {
    let input = &b"quote TESTEX FOOBAR"[..];
    assert_eq!(quote(input).unwrap(), (&b""[..], ("TESTEX", "FOOBAR")));

    let cfg = copperline::Config {
        encoding: copperline::Encoding::Utf8,
        mode: copperline::EditMode::Vi
    };

    let mut cl = Copperline::new();
    while let Ok(line) = cl.read_line(">> ", &cfg) {
        cl.add_history(line.clone());
        let res = quote(line.as_bytes()).unwrap();
        let (stock, venue) = res.1;

        let sf = Stockfighter::new("test");
        let quote = sf.quote(stock, venue);

        println!("quote = {:?}", quote);
    }
}
