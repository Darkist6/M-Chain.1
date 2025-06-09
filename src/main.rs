use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::process::{Command, exit};
use std::fs::{self, File};
use std::io::{Write, BufReader};
use std::path::Path;
use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    nonce: u64,
    previous_hash: String,
    hash: String,
    mining_duration_ms: u128,
}

/// MChain CLI Miner
#[derive(Parser, Debug)]
#[command(name = "mchain")]
#[command(about = "Mine blocks on Apple Silicon with dynamic PoW")]
struct Args {
    /// Number of blocks to mine
    #[arg(short, long, default_value_t = 5)]
    blocks: u64,

    /// Starting difficulty
    #[arg(short = 'l', long, default_value_t = 4)]
    difficulty: usize,

    /// Custom data
    #[arg(short, long, default_value = "MChain data")]
    data: String,
}

fn is_apple_silicon() -> bool {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()
        .expect("Failed to execute sysctl");
    let cpu_info = String::from_utf8_lossy(&output.stdout);
    cpu_info.contains("Apple M")
}

fn calculate_hash(index: u64, timestamp: u64, data: &str, nonce: u64, previous_hash: &str) -> String {
    let input = format!("{}{}{}{}{}", index, timestamp, data, nonce, previous_hash);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn mine_block(index: u64, data: &str, previous_hash: &str, difficulty: usize) -> Block {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut nonce = 0;
    let prefix = "0".repeat(difficulty);
    let start = Instant::now();

    loop {
        let hash = calculate_hash(index, timestamp, data, nonce, previous_hash);
        if hash.starts_with(&prefix) {
            let elapsed = start.elapsed().as_millis();
            println!("✅ Block {} mined in {} ms! Nonce: {}, Hash: {}", index, elapsed, nonce, hash);
            return Block {
                index,
                timestamp,
                data: data.to_string(),
                nonce,
                previous_hash: previous_hash.to_string(),
                hash,
                mining_duration_ms: elapsed,
            };
        }
        nonce += 1;
    }
}

fn save_block_to_file(block: &Block) {
    let dir = Path::new("mchain_data");
    if !dir.exists() {
        fs::create_dir_all(dir).expect("Failed to create mchain_data");
    }
    let path = format!("mchain_data/block_{}.json", block.index);
    let json = serde_json::to_string_pretty(block).expect("Serialize fail");
    let mut file = File::create(path).expect("File write fail");
    file.write_all(json.as_bytes()).expect("Write fail");
}

fn load_blocks_from_disk() -> Vec<Block> {
    let mut chain = Vec::new();
    let path = Path::new("mchain_data");
    if !path.exists() {
        return chain;
    }

    let mut files: Vec<_> = fs::read_dir(path)
        .expect("Read dir fail")
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().map(|e| e == "json").unwrap_or(false))
        .collect();

    files.sort_by_key(|f| f.path());

    for file in files {
        let reader = BufReader::new(File::open(file.path()).expect("Open fail"));
        if let Ok(block) = serde_json::from_reader(reader) {
            chain.push(block);
        }
    }

    // Verify chain
    for i in 1..chain.len() {
        if chain[i].previous_hash != chain[i - 1].hash {
            println!("❌ Chain broken at block {}", chain[i].index);
            chain.truncate(i);
            break;
        }
    }

    chain
}

fn main() {
    if !is_apple_silicon() {
        println!("🚫 MChain mining is only allowed on Apple Silicon.");
        exit(1);
    }

    let args = Args::parse();
    let mut blockchain = load_blocks_from_disk();

    if blockchain.is_empty() {
        println!("⛏️ No existing chain found — creating genesis block...");
        let genesis = mine_block(0, "Genesis Block", "0", args.difficulty);
        save_block_to_file(&genesis);
        blockchain.push(genesis);
    } else {
        println!("🔁 Loaded {} blocks from disk. Resuming chain...", blockchain.len());
    }

    let last_index = blockchain.last().unwrap().index;
    let mut next_index = last_index + 1;

    for i in 0..args.blocks {
        let prev_hash = &blockchain.last().unwrap().hash;
        let difficulty = args.difficulty + ((next_index as usize) / 2);
        let data = format!("{} #{}", args.data, next_index);
        let block = mine_block(next_index, &data, prev_hash, difficulty);
        save_block_to_file(&block);
        blockchain.push(block);
        next_index += 1;
    }

    println!("\n📊 Final Blockchain Summary:");
    for block in &blockchain {
        println!(
            "Block {} | Nonce: {} | Time: {}ms | Hash: {}",
            block.index, block.nonce, block.mining_duration_ms, block.hash
        );
    }
}
