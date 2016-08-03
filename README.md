# Stockfighter App

Example stockfighter.io application that has an interactive command prompt.

## Example

```shell
$ cargo run
     Running `target/debug/stockfighter`
>> quote TESTEX FOOBAR
quote = Ok(Quote { ok: true, symbol: "FOOBAR", venue: "TESTEX", bid: Some(11000), ask: Some(12000), bidSize: Some(16555), askSize: Some(60), bidDepth: Some(16705), askDepth: Some(60), last: 11000, lastSize: Some(10), lastTrade: Some("2016-08-01T22:46:03.66499419Z"), quoteTime: Some("2016-08-02T01:06:31.341003908Z") })
```
