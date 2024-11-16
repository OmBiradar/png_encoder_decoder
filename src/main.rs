use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "PNG Message Encoder/Decoder")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Encodes and decodes messages in PNG files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a message into a PNG image
    Encode {
        /// Path to the input PNG image
        input: String,
        /// Path to save the output encoded PNG image
        output: String,
        /// Message to encode
        message: String,
    },
    /// Decode a message from a PNG image
    Decode {
        /// Path to the encoded PNG image
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode { input, output, message } => {
            if let Err(e) = encode_message(input, output, message) {
                eprintln!("Error encoding message: {}", e);
            }
        }
        Commands::Decode { input } => {
            match decode_message(input) {
                Ok(message) => println!("Decoded message: {}", message),
                Err(e) => eprintln!("Error decoding message: {}", e),
            }
        }
    }
}

fn encode_message(input_path: &str, output_path: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = image::open(input_path)?.into_rgba8();

    let bytes = message.as_bytes();
    let mut byte_index = 0;
    let mut bit_index = 0;

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        if byte_index >= bytes.len() {
            break;
        }

        let byte = bytes[byte_index];
        let bit = (byte >> (7 - bit_index)) & 1;
        pixel[0] = (pixel[0] & 0b11111110) | bit;

        bit_index += 1;
        if bit_index == 8 {
            bit_index = 0;
            byte_index += 1;
        }
    }

    img.save(output_path)?;
    Ok(())
}

fn decode_message(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let img = image::open(input_path)?.into_rgba8();
    let mut bytes = Vec::new();
    let mut byte = 0;
    let mut bit_index = 0;

    for (_, _, pixel) in img.enumerate_pixels() {
        let bit = pixel[0] & 1;
        byte = (byte << 1) | bit;
        bit_index += 1;

        if bit_index == 8 {
            bytes.push(byte);
            if byte == 0 {
                break;
            }
            byte = 0;
            bit_index = 0;
        }
    }

    let message = String::from_utf8(bytes)?;
    Ok(message)
}
