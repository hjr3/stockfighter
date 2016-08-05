#[macro_use]
extern crate nom;
extern crate copperline;
extern crate stockfighter;
extern crate env_logger;

use nom::{IResult, space, alphanumeric, digit};
use copperline::Copperline;
use stockfighter::{Stockfighter, OrderDirection, OrderType};

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Check,
    Quote,
    ListStocks,
    OrderBook,
    OrderBuy,
    OrderSell,
    StatusOrder,
    CancelOrder,
    TickerTape,
    Unknown,
}

// Parse a numerical array into a string and then from a string into a number
named!(usize_digit<usize>,
    map_res!(
        map_res!(
            digit,
            std::str::from_utf8
        ),
        std::str::FromStr::from_str
    )
);

named!(an_string<&str>,
    map_res!(
        alphanumeric,
        std::str::from_utf8
    )
);

named!(commands<&[u8], Command>,
    alt!(
        tag!("check") => { |_| Command::Check } |
        tag!("quote") => { |_| Command::Quote } |
        tag!("list") => { |_| Command::ListStocks } |
        tag!("book") => { |_| Command::OrderBook } |
        tag!("buy") => { |_| Command::OrderBuy } |
        tag!("sell") => { |_| Command::OrderSell } |
        tag!("status") => { |_| Command::StatusOrder } |
        tag!("cancel") => { |_| Command::CancelOrder } |
        tag!("ticker") => { |_| Command::TickerTape }
    )
);

named!(venue_stock<&[u8], (&str, &str)>,
    chain!(
        space? ~
        venue: an_string ~
        space? ~
        stock: an_string ,

        || (venue, stock)
    )
);

named!(account_venue_stock<&[u8], (&str, &str, &str)>,
    chain!(
        space? ~
        account: an_string ~
        space? ~
        venue: an_string ~
        space? ~
        stock: an_string ,

        || (account, venue, stock)
    )
);

named!(order_buy<&[u8], (&str, &str, &str, usize, usize)>,
    chain!(
        space? ~
        account: an_string ~
        space? ~
        venue: an_string ~
        space? ~
        stock: an_string ~
        space? ~
        price: usize_digit ~
        space? ~
        qty: usize_digit ,

        || (account, venue, stock, price, qty)
    )
);

fn handle(line: &str) {
    let (args, command) = match commands(line.as_bytes()) {
        IResult::Done(i, o) => (i, o),
        IResult::Error(_) => (&b""[..], Command::Unknown),
        IResult::Incomplete(_) => (&b""[..], Command::Unknown),
    };

    let sf = Stockfighter::new("b6eb6d0a2b606c02c8b027fca35383fb2dc741d3");
    match command {
        Command::Quote => {
            let (_, (venue, stock)) = venue_stock(args).unwrap();
            let quote = sf.quote(venue, stock);
            println!("quote = {:?}", quote);
        }
        Command::TickerTape => {
            let (_, (account, venue, stock)) = account_venue_stock(args).unwrap();
            let handle = sf.ticker_tape_venue_stock_with(account, venue, stock, |quote| println!("{:?}", quote));
            let _ = handle.unwrap().join();
        }
        Command::OrderBuy => {
            match order_buy(args) {
                IResult::Done(_, (account, venue, stock, price, qty)) => {
                    let order = sf.new_order(account, venue, stock, price, qty, OrderDirection::buy, OrderType::Limit).unwrap();
                    println!("order = {:?}", order);
                }
                IResult::Error(_) => {
                    panic!("Error parsing arguments for OrderBuy");
                }
                IResult::Incomplete(i) => {
                    println!("Syntax error: {:?}", i);
                }
            }
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
    env_logger::init().expect("Failed to start logger");

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
