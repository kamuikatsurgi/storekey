mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{CliArgs, CommandResult};

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();

    match cli_args.command.execute()? {
        CommandResult::EncryptResult(encrypted_data) => {
            println!("Encrypted Data: {}", encrypted_data);
        }
        CommandResult::DecryptResult(decrypted_data) => {
            println!("Decrypted Seed Phrase: {}", decrypted_data);
        }
    }

    Ok(())
}
