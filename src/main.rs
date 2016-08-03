#[macro_use]
extern crate nom;
extern crate copperline;
extern crate stockfighter;

use nom::{IResult, space, alphanumeric};
use copperline::Copperline;
use stockfighter::Stockfighter;

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Check,
    Quote,
    ListStocks,
    OrderBook,
    NewOrder,
    StatusOrder,
    CancelOrder,
    Unknown,
}

named!(commands<&[u8], Command>,
    alt!(
        tag!("check") => { |_| Command::Check } |
        tag!("quote") => { |_| Command::Quote } |
        tag!("list") => { |_| Command::ListStocks } |
        tag!("book") => { |_| Command::OrderBook } |
        tag!("new") => { |_| Command::NewOrder } |
        tag!("status") => { |_| Command::StatusOrder } |
        tag!("cancel") => { |_| Command::CancelOrder }
    )
);

named!(quote<&[u8], (&str,&str)>,
    chain!(
        space? ~
        venue: map_res!(alphanumeric, std::str::from_utf8) ~
        space? ~
        stock: map_res!(alphanumeric, std::str::from_utf8) ,

        || (venue, stock)
    )
);

fn handle(line: &str) {
    let (args, command) = match commands(line.as_bytes()) {
        IResult::Done(i, o) => (i, o),
        IResult::Error(_) => (&b""[..], Command::Unknown),
        IResult::Incomplete(_) => (&b""[..], Command::Unknown),
    };

    let sf = Stockfighter::new("test");
    match command {
        Command::Quote => {
            let (_, (stock, venue)) = quote(args).unwrap();
            let quote = sf.quote(stock, venue);
            println!("quote = {:?}", quote);
        }
        Command::Unknown => {
            println!("Command '{}' is not known", line);
        }
        _ => {
            println!("Command '{}' not supported", line);
        }
    }
}


fn main() {
    let cfg = copperline::Config {
        encoding: copperline::Encoding::Utf8,
        mode: copperline::EditMode::Vi
    };

    let mut cl = Copperline::new();
    while let Ok(line) = cl.read_line(">> ", &cfg) {
        handle(&line);
        cl.add_history(line);
    }
}

#[test]
fn test_commands() {
    let empty = &b""[..];

    let inputs = vec![
        ("check", Command::Check),
        ("quote", Command::Quote),
        ("list", Command::ListStocks),
        ("book", Command::OrderBook),
        ("new", Command::NewOrder),
        ("status", Command::StatusOrder),
        ("cancel", Command::CancelOrder),
    ];

    for (given, expected) in inputs {
        assert_eq!(commands(given.as_bytes()), IResult::Done(empty, expected));
    }

    assert!(commands("foo".as_bytes()).is_err());
}

#[test]
fn test_quote() {
    let empty = &b""[..];
    assert_eq!(quote(" TESTEX FOOBAR".as_bytes()), IResult::Done(empty, ("TESTEX", "FOOBAR")));
}
