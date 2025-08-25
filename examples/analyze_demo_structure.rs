use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Example to analyze the structure of a CS2 demo file
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CS2 Demo Structure Analyzer");
    println!("===========================");

    let demo_path = "examples/data/sample.dem";
    
    if !Path::new(demo_path).exists() {
        println!("❌ Demo file not found at: {}", demo_path);
        return Ok(());
    }

    println!("📁 Analyzing file: {}", demo_path);
    
    // Open and analyze the demo file
    let mut file = File::open(demo_path)?;
    let file_size = file.metadata()?.len();
    println!("📊 File size: {} bytes ({:.2} MB)", file_size, file_size as f64 / 1024.0 / 1024.0);

    // Read and analyze the header (first 1024 bytes)
    let header_size = 1024.min(file_size as usize);
    let mut header = vec![0u8; header_size];
    file.read_exact(&mut header)?;

    println!("\n🔍 Header Analysis (first {} bytes):", header_size);
    println!("==========================================");

    // Show first 64 bytes as hex
    println!("First 64 bytes (hex):");
    for (i, chunk) in header[..64.min(header_size)].chunks(16).enumerate() {
        let hex: String = chunk.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
        let ascii: String = chunk.iter().map(|&b| if b.is_ascii_graphic() { b as char } else { '.' }).collect();
        println!("{:04x}: {:48} |{}|", i * 16, hex, ascii);
    }

    // Try to identify file format
    println!("\n🔍 Format Analysis:");
    println!("===================");

    // Check for common demo file signatures
    if header.len() >= 8 {
        let signature = &header[0..8];
        println!("Signature (first 8 bytes): {:?}", signature);
        
        // Check if it looks like a protobuf file
        if signature.iter().any(|&b| b == 0x08 || b == 0x10 || b == 0x18) {
            println!("✅ Possible protobuf format detected");
        }
        
        // Check for CS2 demo magic bytes (if known)
        if signature.starts_with(b"HL2DEMO") {
            println!("✅ CS2 demo format detected");
        }
    }

    // Analyze byte patterns
    println!("\n📊 Byte Pattern Analysis:");
    println!("=========================");
    
    let mut byte_counts = [0u32; 256];
    for &byte in &header {
        byte_counts[byte as usize] += 1;
    }

    // Show most common bytes
    let mut common_bytes: Vec<(u8, u32)> = byte_counts.iter().enumerate()
        .map(|(byte, &count)| (byte as u8, count))
        .filter(|(_, count)| *count > 0)
        .collect();
    common_bytes.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Most common bytes in header:");
    for (byte, count) in common_bytes.iter().take(10) {
        let percentage = (*count as f64 / header_size as f64) * 100.0;
        println!("  0x{:02x} ({:3}): {:4} times ({:.1}%)", 
                byte, *byte as char, count, percentage);
    }

    // Look for potential string data
    println!("\n🔤 String Analysis:");
    println!("===================");
    
    let mut current_string = Vec::new();
    let mut strings = Vec::new();
    
    for &byte in &header {
        if byte.is_ascii_graphic() && byte != 0 {
            current_string.push(byte);
        } else {
            if current_string.len() >= 3 {
                if let Ok(s) = String::from_utf8(current_string.clone()) {
                    strings.push(s);
                }
            }
            current_string.clear();
        }
    }
    
    // Check for remaining string
    if current_string.len() >= 3 {
        if let Ok(s) = String::from_utf8(current_string) {
            strings.push(s);
        }
    }

    println!("Potential strings found (length >= 3):");
    for string in strings.iter().take(20) {
        println!("  \"{}\"", string);
    }

    // Try to find potential offsets or pointers
    println!("\n📍 Offset Analysis:");
    println!("===================");
    
    let mut potential_offsets = Vec::new();
    for i in 0..header.len().saturating_sub(4) {
        let offset = u32::from_le_bytes([header[i], header[i+1], header[i+2], header[i+3]]);
        if offset > 0 && u64::from(offset) < file_size {
            potential_offsets.push((i, offset));
        }
    }

    println!("Potential 32-bit offsets (little-endian):");
    for (pos, offset) in potential_offsets.iter().take(10) {
        println!("  Position 0x{:04x}: 0x{:08x} ({})", pos, offset, offset);
    }

    // Look for potential message boundaries
    println!("\n📨 Message Boundary Analysis:");
    println!("=============================");
    
    let mut message_starts = Vec::new();
    for i in 0..header.len().saturating_sub(4) {
        // Look for potential protobuf field headers
        let byte = header[i];
        if (byte & 0x07) <= 5 && (byte >> 3) > 0 && (byte >> 3) <= 16 {
            message_starts.push(i);
        }
    }

    println!("Potential protobuf field headers found: {} positions", message_starts.len());
    for &pos in message_starts.iter().take(10) {
        let field_number = header[pos] >> 3;
        let wire_type = header[pos] & 0x07;
        println!("  Position 0x{:04x}: field={}, wire_type={}", pos, field_number, wire_type);
    }

    println!("\n✅ Analysis complete!");
    println!("💡 Next steps: Implement protobuf parsing based on these patterns");

    Ok(())
}
