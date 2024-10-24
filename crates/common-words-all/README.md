# common-words-all

Most common words sorted by ngram frequency. 

Available in the following languages:

- Chinese
- English
- French
- German
- Hebrew
- Italian
- Russian
- Spanish

Available ngram sizes:

- 1
- 2
- 3
- 4
- 5

## Usage

Get top 10 english ngrams:

```rust
let top = get_top(Language::English, 10, NgramSize::One);
```

## Examples

### Simple

You can specify features of language ()`english`) and ngram size (`one`)

```shell
cargo run --example simple --no-default-features -F english -F one --release
```

## Data

Dataset version 20200217 from [Google Books](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)

## License

MIT

## Copyright

© 2024, Eugene Hauptmann