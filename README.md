# PNG encoder and decoder

This tool can encode a message into a PNG image by modifying the Red Channel.

This tool is written in rust🦀!!!!

You can use the help command to see all the availble commands and details about them in the terminal.

> Note : You can use the exe file after generating it, until that we can stick with `cargo run --` after which we can pass the arguments just like we would pass it to the final executable.

```
cargo run -- help
```

The output is shown below
```
Encodes and decodes messages in PNG files

Usage: png_encoder_decoder.exe <COMMAND>

Commands:
  encode  Encode a message into a PNG image
  decode  Decode a message from a PNG image
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Encoding

Pass the path to the image to be encoded and the path of the output image along with the message that you want to encode into the final image.
```
cargo run -- encode input.png output.png "Hey, it's me Om!"
```

### Decoding 

Pass the path to the image to decode and it will show the message.

```
cargo run -- decode output.png
```