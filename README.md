# Solana KeyTool

A command-line utility to convert Solana keypairs between the JSON format used by the Solana CLI (`id.json`) and the base58-encoded string format for private keys.

## ⚠️ Security Warning

This tool is intended for development and educational purposes. **Do not use it with mainnet private keys in a non-secure environment.**

-   **Command-line History**: Passing a private key as a command-line argument (like with the `from-base58` command) can cause the key to be stored in your shell's history file (e.g., `.bash_history`, `.zsh_history`) in plain text.
-   **Console Output**: Printing a private key to the console (like with the `to-base58` command) can make it visible to anyone looking over your shoulder and it might be logged if you are redirecting output.

An attacker who gains access to your computer could potentially steal your keys from these sources. Use this tool with caution and preferably with throwaway development keys.

## Description

This tool provides two main functionalities:

1.  **Encode to Base58**: Reads a Solana keypair from a JSON file (e.g., `~/.config/solana/id.json`) and prints the corresponding base58-encoded private key.
2.  **Decode from Base58**: Takes a base58-encoded private key and saves it as a JSON-formatted keypair file.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust's package manager).

## Building

1.  Clone the repository:
    ```bash
    git clone <repository-url>
    cd solana-keytool
    ```

2.  Build the project:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/solana-keytool`.

## Usage

You can run the tool using `cargo run --` followed by the desired command and arguments.

### `to-base58`

Encodes a 64-byte array from a JSON file to a base58 private key.

**Command:**

```bash
cargo run -- to-base58 --input <path-to-your-id.json>
```

**Arguments:**

-   `--input <path>` or `-i <path>`: Specifies the path to the input JSON keypair file. Defaults to `id.json` if not provided.

**Example:**

```bash
# Read from the default solana cli keypair location
cargo run -- to-base58 --input ~/.config/solana/id.json

# Read from a file named my-wallet.json in the current directory
cargo run -- to-base58 --input my-wallet.json
```

### `from-base58`

Decodes a base58 private key and saves it as a JSON file.

**Command:**

```bash
cargo run -- from-base58 <base58-private-key> --output <path-to-output.json>
```

**Arguments:**

-   `<base58-private-key>`: (Required) The base58-encoded private key string.
-   `--output <path>` or `-o <path>`: Specifies the path for the output JSON file. Defaults to `id.json` if not provided.

**Example:**

```bash
# Decode a key and save it to the default id.json
cargo run -- from-base58 5J2J...YourPrivateKey...t5sP

# Decode a key and save it to a specific file
cargo run -- from-base58 5J2J...YourPrivateKey...t5sP --output my-new-wallet.json
```

