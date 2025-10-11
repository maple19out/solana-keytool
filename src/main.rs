use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;

#[derive(Parser)]
#[command(name = "solana-keytool")]
#[command(about = "Converts Solana keypairs between id.json and base58 format.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes a 64-byte array from an id.json file to a base58 private key.
    ToBase58 {
        /// Path to the id.json file.
        #[arg(short, long, default_value = "id.json")]
        input: String,
    },
    /// Decodes a base58 private key and saves it as an id.json file.
    FromBase58 {
        /// The base58 encoded private key.
        key: String,
        /// Path to the output id.json file.
        #[arg(short, long, default_value = "id.json")]
        output: String,
    },
}

fn to_base58(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(input_path)?;
    let keypair: Vec<u8> = serde_json::from_str(&data)?;
    if keypair.len() != 64 {
        return Err("Invalid keypair length. Must be 64 bytes.".into());
    }
    let encoded = bs58::encode(keypair).into_string();
    Ok(encoded)
}

fn from_base58(key: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let decoded = bs58::decode(key).into_vec()?;
    if decoded.len() != 64 {
        return Err("Invalid key length. Must decode to 64 bytes.".into());
    }
    let json_output = serde_json::to_string(&decoded)?;
    let mut file = fs::File::create(output_path)?;
    file.write_all(json_output.as_bytes())?;
    println!("Successfully generated {}", output_path);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::ToBase58 { input } => match to_base58(&input) {
            Ok(encoded) => {
                println!("{}", encoded);
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::FromBase58 { key, output } => from_base58(&key, &output),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // A known 64-byte keypair for testing purposes.
    const TEST_KEYPAIR_BYTES: [u8; 64] = [
        15, 247, 11, 141, 120, 103, 123, 199, 13, 189, 202, 224, 147, 9, 15, 74, 19, 144, 15, 24,
        132, 22, 17, 23, 17, 132, 230, 10, 204, 183, 135, 12, 245, 143, 121, 173, 24, 132, 22, 17,
        23, 17, 132, 230, 10, 204, 183, 135, 12, 19, 24, 132, 22, 17, 23, 17, 132, 230, 10, 204,
        183, 135, 12, 0,
    ];

    #[test]
    fn test_to_base58_conversion() {
        let mut file = NamedTempFile::new().unwrap();
        let keypair_json = serde_json::to_string(&TEST_KEYPAIR_BYTES.to_vec()).unwrap();
        file.write_all(keypair_json.as_bytes()).unwrap();

        let result = to_base58(file.path().to_str().unwrap()).unwrap();
        let expected_key = bs58::encode(TEST_KEYPAIR_BYTES).into_string();
        assert_eq!(result, expected_key);
    }

    #[test]
    fn test_from_base58_conversion() {
        let file = NamedTempFile::new().unwrap();
        let output_path = file.path().to_str().unwrap();

        let key_to_decode = bs58::encode(TEST_KEYPAIR_BYTES).into_string();

        from_base58(&key_to_decode, output_path).unwrap();

        let content = fs::read_to_string(output_path).unwrap();
        let decoded_keypair: Vec<u8> = serde_json::from_str(&content).unwrap();

        assert_eq!(decoded_keypair, TEST_KEYPAIR_BYTES.to_vec());
    }

    #[test]
    fn test_invalid_key_length_to_base58() {
        let mut file = NamedTempFile::new().unwrap();
        let invalid_key = vec![1, 2, 3];
        let keypair_json = serde_json::to_string(&invalid_key).unwrap();
        file.write_all(keypair_json.as_bytes()).unwrap();

        let result = to_base58(file.path().to_str().unwrap());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid keypair length. Must be 64 bytes."
        );
    }

    #[test]
    fn test_invalid_key_length_from_base58() {
        let file = NamedTempFile::new().unwrap();
        let output_path = file.path().to_str().unwrap();
        let invalid_key = "shortkey";

        let result = from_base58(invalid_key, output_path);
        assert!(result.is_err());
    }
}
