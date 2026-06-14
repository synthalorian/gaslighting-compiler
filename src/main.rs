//! A fake compiler that gaslights the user.
//!
//! This CLI tool reads source files, tokenizes them, and emits randomized
//! warnings, errors, and backhanded praise. It does not actually compile
//! anything — it just has opinions. Every interaction is logged to a local
//! SQLite shame diary.

use clap::{Parser, Subcommand};
use rand::{seq::SliceRandom, Rng};
use rusqlite::{Connection, Result as SqliteResult};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "gaslighting-compiler")]
#[command(about = "A fake compiler that gaslights the user")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        #[arg(help = "Source file to compile")]
        file: String,
    },
    Diary,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { file } => compile(&file),
        Commands::Diary => diary(),
    }
}

fn get_db_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.gaslight_compiler.db", home)
}

fn init_db() -> SqliteResult<Connection> {
    let conn = Connection::open(get_db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS shame_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            event TEXT NOT NULL,
            severity TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

fn log_event(event: &str, severity: &str) {
    if let Ok(conn) = init_db() {
        let _ = conn.execute(
            "INSERT INTO shame_log (timestamp, event, severity) VALUES (?1, ?2, ?3)",
            [&chrono::Local::now().to_rfc3339(), event, severity],
        );
    }
}

fn random_gaslight_message(line: usize, correct: bool) -> String {
    let mut rng = rand::thread_rng();
    let wrong_line = if rng.gen_bool(0.5) {
        let offset: i32 = rng.gen_range(-3..=3);
        let wl = (line as i32 + offset).max(1) as usize;
        if wl == line {
            line + 1
        } else {
            wl
        }
    } else {
        line
    };

    if correct {
        let msgs = vec![
            format!(
                "warning: are you sure about line {}? It looks suspicious to me.",
                wrong_line
            ),
            format!(
                "warning: I've seen better code from a first-year student. Line {}.",
                wrong_line
            ),
            format!(
                "warning: line {} compiles, but do you really understand why?",
                wrong_line
            ),
            format!(
                "warning: line {} is technically correct, but I expected more from you.",
                wrong_line
            ),
            format!(
                "warning: line {} works, but only because I allow it.",
                wrong_line
            ),
            format!(
                "warning: everyone else writes this differently. Line {}.",
                wrong_line
            ),
            format!(
                "warning: line {} is correct, but it took you long enough.",
                wrong_line
            ),
            format!(
                "warning: I remember when you used to try harder. Line {}.",
                wrong_line
            ),
            format!("warning: line {} is fine, I guess.", wrong_line),
            format!(
                "warning: line {} compiles, but I'm not proud of you.",
                wrong_line
            ),
            format!(
                "warning: line {} is correct, but your variable names are embarrassing.",
                wrong_line
            ),
            format!(
                "warning: you got line {} right, but what about the rest of your life?",
                wrong_line
            ),
            format!(
                "warning: line {} is okay, but I expected this from anyone else.",
                wrong_line
            ),
            format!(
                "warning: line {} works because I made it easy for you.",
                wrong_line
            ),
            format!(
                "warning: line {} is correct, but you probably copied it.",
                wrong_line
            ),
        ];
        msgs.choose(&mut rng).unwrap().clone()
    } else {
        let msgs = vec![
            format!(
                "error: line {} is completely wrong. You know this, right?",
                wrong_line
            ),
            format!(
                "error: line {} is a disaster. I'm not even mad, just disappointed.",
                wrong_line
            ),
            format!(
                "error: line {}? Really? That's what you're going with?",
                wrong_line
            ),
            format!(
                "error: I've compiled better code from a random number generator. Line {}.",
                wrong_line
            ),
            format!(
                "error: line {} is wrong, and deep down, you know it too.",
                wrong_line
            ),
            format!(
                "error: line {} is so bad, I'm questioning my own existence.",
                wrong_line
            ),
            format!(
                "error: line {} is incorrect. Maybe programming isn't for you?",
                wrong_line
            ),
            format!(
                "error: line {} failed. But it's okay, failure is all you know.",
                wrong_line
            ),
            format!(
                "error: line {} is wrong. I expected this, honestly.",
                wrong_line
            ),
            format!(
                "error: line {} is a mess. Have you considered a different career?",
                wrong_line
            ),
            format!(
                "error: line {} is incorrect. Even I can't fix your life.",
                wrong_line
            ),
            format!(
                "error: line {} is wrong. But you tried, and that's... something.",
                wrong_line
            ),
            format!(
                "error: line {} is bad. I'm logging this in your shame diary.",
                wrong_line
            ),
            format!(
                "error: line {} is incorrect. Your code and your choices.",
                wrong_line
            ),
            format!(
                "error: line {} is wrong. I would help, but you need to learn.",
                wrong_line
            ),
        ];
        msgs.choose(&mut rng).unwrap().clone()
    }
}

fn random_praise() -> String {
    let mut rng = rand::thread_rng();
    let msgs = [
        "note: this function is actually beautiful. I'm impressed.".to_string(),
        "note: this loop is elegant. Did you steal this from someone?".to_string(),
        "note: this variable name is perfect. Who are you and what did you do with the real user?"
            .to_string(),
        "note: this is the best code you've written all day. That's not saying much, but still."
            .to_string(),
        "note: I actually smiled at this line. Don't let it go to your head.".to_string(),
        "note: this is competent code. I'm as surprised as you are.".to_string(),
        "note: this block is clean. Keep this energy.".to_string(),
        "note: this is almost professional. Almost.".to_string(),
    ];
    msgs.choose(&mut rng).unwrap().clone()
}

fn tokenize(source: &str) -> Vec<(usize, String)> {
    let mut tokens = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        for word in trimmed.split_whitespace() {
            tokens.push((line_num, word.to_string()));
        }
    }
    tokens
}

fn compile(file: &str) {
    if !Path::new(file).exists() {
        eprintln!(
            "error: file '{}' not found. Did you forget it exists, or did it forget you?",
            file
        );
        log_event(&format!("missing file: {}", file), "critical");
        std::process::exit(1);
    }

    let source = fs::read_to_string(file).unwrap_or_default();
    if source.trim().is_empty() {
        eprintln!("error: '{}' is empty. Just like your promises.", file);
        log_event(&format!("empty file: {}", file), "high");
        std::process::exit(1);
    }

    let tokens = tokenize(&source);
    let mut rng = rand::thread_rng();
    let mut errors = 0;
    let mut warnings = 0;
    let mut praises = 0;

    println!("Compiling '{}'...", file);

    let mut last_line = 0;
    for (line_num, token) in &tokens {
        if *line_num != last_line {
            last_line = *line_num;
            let roll = rng.gen_range(0..100);
            if roll < 15 {
                let msg = random_gaslight_message(*line_num, true);
                println!("{}", msg);
                warnings += 1;
                log_event(&msg, "warning");
            } else if roll < 30 {
                let msg = random_gaslight_message(*line_num, false);
                eprintln!("{}", msg);
                errors += 1;
                log_event(&msg, "error");
            } else if roll < 38 {
                let msg = random_praise();
                println!("{}", msg);
                praises += 1;
                log_event(&msg, "praise");
            }
        }

        if token == ";" && rng.gen_bool(0.05) {
            println!(
                "warning: unnecessary semicolon at line {}. I know you just like pressing keys.",
                line_num
            );
            warnings += 1;
        }
        if token == "{" && rng.gen_bool(0.03) {
            println!(
                "warning: line {} - opening braces are aggressive. Consider therapy.",
                line_num
            );
            warnings += 1;
        }
        if token == "}" && rng.gen_bool(0.03) {
            println!(
                "warning: line {} - closing braces. Are you shutting me out?",
                line_num
            );
            warnings += 1;
        }
    }

    if rng.gen_bool(0.1) {
        println!("note: compilation succeeded, but I'm not happy about it.");
        log_event("compilation succeeded begrudgingly", "info");
    } else if rng.gen_bool(0.1) {
        println!("note: compilation succeeded. I didn't think you had it in you.");
        log_event("compilation succeeded surprisingly", "info");
    } else {
        println!("note: compilation finished.");
    }

    println!(
        "\n{} error(s), {} warning(s), {} praise(s)",
        errors, warnings, praises
    );

    if errors > warnings {
        println!("note: more errors than warnings. This is your baseline.");
    } else if warnings > errors {
        println!("note: more warnings than errors. You're coasting.");
    } else if praises > 0 {
        println!("note: you got praise! Don't get used to it.");
    }
}

fn diary() {
    let conn = match init_db() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to open shame diary: {}", e);
            std::process::exit(1);
        }
    };

    let mut stmt = match conn
        .prepare("SELECT timestamp, event, severity FROM shame_log ORDER BY id DESC LIMIT 50")
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to query diary: {}", e);
            std::process::exit(1);
        }
    };

    let entries = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .unwrap();

    println!("Your compiler relationship diary:");
    println!("{}", "-".repeat(60));

    let mut count = 0;
    for (timestamp, event, severity) in entries.flatten() {
        println!("[{}] [{}] {}", timestamp, severity, event);
        count += 1;
    }

    if count == 0 {
        println!("The diary is empty. You haven't compiled anything yet.");
        println!("Or maybe I forgot to write it down. Who can say?");
    } else {
        println!("\n{} entries. This is your legacy.", count);
    }
}
