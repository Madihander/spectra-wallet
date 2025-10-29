use spectra_wallet::{Wallet, WalletBuilder, print_banner};

use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::path::PathBuf;

pub fn run_cli() -> Result<()> {
    print_banner();

    let wallet_path = PathBuf::from("wallet.json");

    if !wallet_path.exists() {
        println!("{}", "⚠️  Wallet not found!".yellow());
        println!("Let's create a new wallet 🔐\n");

        let options = &["🪄 Create new wallet", "❌ Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(options)
            .interact()?;

        match selection {
            0 => {
                create_wallet(&wallet_path)?;
                wallet_menu(&wallet_path)?; 
            },
            1 => {
                println!("👋 Goodbye!");
                return Ok(());
            }
            _ => {}
        }
    } else {
        println!("{}", "✅ Wallet file found.".green());
        let options = &["🔓 Open wallet", "🧩 Recover wallet", "❌ Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(options)
            .interact()?;

        match selection {
            0 => {
                // просто открываем текущее меню
                wallet_menu(&wallet_path)?;
            }
            1 => {
                // восстанавливаем кошелёк на основе seed_colors
                recover_existing_wallet(&wallet_path)?;
                wallet_menu(&wallet_path)?;
            }
            2 => {
                println!("👋 Goodbye!");
                return Ok(());
            }
            _ => {}
        }
    }

    Ok(())
}

fn create_wallet(wallet_path: &PathBuf) -> anyhow::Result<()> {
    use dialoguer::{Input, theme::ColorfulTheme};
    use colored::*;
    let theme = ColorfulTheme::default();

    let image_path = loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Enter path to your image")
            .interact_text()?;

        let path = PathBuf::from(input.trim());
        if path.exists() && path.is_file() {
            break path;
        } else {
            println!("{}", "❌ File does not exist, try again.".red());
        }
    };

    let emoji = loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Enter emoji (example: 🚀, 🧠, 🦋)")
            .interact_text()?;
        let input = input.trim();
        if !input.is_empty() {
            break input.to_string();
        } else {
            println!("{}", "❌ Emoji cannot be empty.".red());
        }
    };

    let color = loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Enter color in HEX (example: #ff00ff)")
            .interact_text()?;
        let input = input.trim();
        if input.starts_with('#') && input.len() == 7 && u32::from_str_radix(&input[1..], 16).is_ok() {
            break input.to_string();
        } else {
            println!("{}", "❌ Invalid HEX color, try again.".red());
        }
    };

    let blockchain = loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Blockchain (default: solana)")
            .allow_empty(true)
            .interact_text()?;
        let input = input.trim();
        
        // Если пусто или solana — используем solana
        if input.is_empty() || input.eq_ignore_ascii_case("solana") {
            break "solana".to_string();
        } else {
            println!("{}", "⚠️ Currently only Solana is supported. It's recommended to use Solana.".yellow());
            break "solana".to_string();
        }
    };

    let wallet = WalletBuilder::generate_wallet(&image_path, &emoji, &color, &blockchain)?;

    wallet.save(wallet_path.to_str().unwrap())?;
    println!(
        "\n{}",
        "✅ Wallet successfully created and saved to wallet.json!".green()
    );

    display_seed_colors(wallet.get_seed_colors());

    Ok(())
}

fn recover_existing_wallet(wallet_path: &PathBuf) -> Result<()> {
    use serde_json::Value;
    use std::fs;

    println!("{}", "🧩 Starting wallet recovery...".cyan());
    println!("{}", "[[Let's omit the moment with the input of seed colors.]]");
    // читаем текущий wallet.json

    let theme = ColorfulTheme::default();
    let data = fs::read_to_string(&wallet_path)?;
    let json: Value = serde_json::from_str(&data)?;

    // достаём seed_colors
    let seed_colors: Vec<String> = json["seed_colors"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    if seed_colors.is_empty() {
        println!("❌ No seed colors found in wallet.json!");
        return Ok(());
    }

    // ввод пути к новой картинке
    let image_path = loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Enter path to your image")
            .interact_text()?;

        let path = PathBuf::from(input.trim());
        if path.exists() && path.is_file() {
            break path;
        } else {
            println!("{}", "❌ File does not exist, try again.".red());
        }
    };

    // ввод emoji
    let emoji = loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Enter emoji (example: 🚀, 🧠, 🦋)")
            .interact_text()?;
        let input = input.trim();
        if !input.is_empty() {
            break input.to_string();
        } else {
            println!("{}", "❌ Emoji cannot be empty.".red());
        }
    };

    // восстанавливаем кошелёк
    let recovered_wallet =
        WalletBuilder::recover_wallet(&seed_colors, &image_path, &emoji)?;

    // удаляем старый wallet.json
    fs::remove_file(&wallet_path)?;
    println!("{}", "🧹 Old wallet.json deleted.".yellow());

    // сохраняем новый
    recovered_wallet.save(wallet_path.to_str().unwrap())?;
    println!("{}", "✅ Wallet successfully recovered and saved!\n".green());

    Ok(())
}

fn wallet_menu(wallet_path: &PathBuf) -> Result<()> {
    let wallet = Wallet::load(wallet_path.to_str().unwrap())?;

    loop {
        let items = &[
            "📬 Show address",
            "🔑 Show public key",
            "🔐 Show private key",
            "🔓 Show decrypted private key",
            "🌈 Show seed colors",
            "🧾 Sign transaction",
            "❌ Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(items)
            .interact()?;

        match selection {
            0 => println!("📬 Address: {}", wallet.address),
            1 => println!("🔑 Public key: {}", wallet.get_hex_pub()),
            2 => println!("🔐 Private key: {}", wallet.get_hex_encrypted_sec()),
            3 => println!("🔓 Decrypted Private key: {}", hex::encode(wallet.decryption_private_key()?)),
            4 => display_seed_colors(wallet.get_seed_colors()),
            5 => sign_transaction(&wallet)?,
            6 => {
                println!("👋 Goodbye!");
                break;
            }
            _ => {}
        }
        println!();
    }

    Ok(())
}


fn sign_transaction(wallet: &Wallet) -> Result<()>{
    use std::io::{self, Write};

    print!("📝 Enter transaction data: ");
    io::stdout().flush().unwrap(); // чтобы сразу показать приглашение
    let mut transaction_data = String::new();
    io::stdin()
        .read_line(&mut transaction_data)
        .expect("Failed to read input");
    let transaction_data = transaction_data.trim();

    if transaction_data.is_empty() {
        println!("⚠️ Transaction data cannot be empty!");
        return Ok(());
    }

    // Подпись данных
    let signature = wallet.sign_transaction(transaction_data.as_bytes())?;
    let signature_hex = hex::encode(&signature);

    println!("🧾 Signature: {}", signature_hex);

    // Проверка подписи
    let is_valid = wallet.verify_signature(transaction_data.as_bytes(), &signature)?;
    if is_valid {
        println!("✅ Signature is valid!");
    } else {
        println!("❌ Signature is invalid!");
    }

    Ok(())

}

fn display_seed_colors(seed_colors: &[String]) {
    println!("🌈 Seed colors:\n");

    let colors_per_row = 5;
    for (i, color_hex) in seed_colors.iter().enumerate() {
        let hex = color_hex.trim_start_matches('#');
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                let block = "    ".on_truecolor(r, g, b);
                // выравнивание: цвет + hex + пробелы
                print!("{} {:<9}", block, color_hex);
            } else {
                print!("{:<13}", color_hex);
            }
        } else {
            print!("{:<13}", color_hex);
        }

        // переход на новую строку каждые 5 цветов
        if (i + 1) % colors_per_row == 0 {
            println!();
        }
    }
    println!("\n");
}