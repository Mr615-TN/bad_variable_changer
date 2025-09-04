use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use regex::Regex;
use std::io;

mod modifyfile;
mod language_patterns;

use language_patterns::LanguagePattern;

fn get_yourmom_variation(index: usize) -> String {
    let variations = vec![
        "yourmom", "yOurMom", "YourMom", "yourMom", "YOURMOM",
        "YouRmOm", "yOuRmOm", "YoUrMoM", "yourmOM", "YOURmom",
        "YoUrMoThEr", "yourmother", "YourMother", "YOURMOTHER",
        "yOuRmOtHeR", "yourmommy", "YourMommy", "YOURMOMMY",
        "urmom", "UrMom", "URMOM", "yomama", "YoMama", "YOMAMA"
    ];
    variations[index % variations.len()].to_string()
}

fn is_bad_variable(var_name: &str) -> bool {
    // Skip if it's already a yourmom variation
    if var_name.to_lowercase().contains("yourmom") || 
       var_name.to_lowercase().contains("yourmother") ||
       var_name.to_lowercase().contains("urmom") ||
       var_name.to_lowercase().contains("yomama") {
        return false;
    }

    let bad_patterns = vec![
        // Single letter variables (a-z, but keep common ones like _ or $)
        Regex::new(r"^[a-z]$").unwrap(),
        // Common bad patterns
        Regex::new(r"^(i|j|k|l|m|n|x|y|z|a|b|c|d|e|f|g|h)$").unwrap(),
        // Generic terrible names
        Regex::new(r"^(temp|tmp|var|val|data|item|elem|node|obj|thing|stuff)$").unwrap(),
        // Variables with just numbers
        Regex::new(r"^[a-z]+\d+$").unwrap(),
        // Really short meaningless names
        Regex::new(r"^(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)$").unwrap(),
    ];
    
    bad_patterns.iter().any(|pattern| pattern.is_match(var_name))
}

fn detect_language(file_path: &Path) -> Option<LanguagePattern> {
    if let Some(extension) = file_path.extension() {
        match extension.to_str()? {
            "rs" => Some(LanguagePattern::Rust),
            "js" | "jsx" | "ts" | "tsx" => Some(LanguagePattern::JavaScript),
            "py" => Some(LanguagePattern::Python),
            "java" => Some(LanguagePattern::Java),
            "cpp" | "cc" | "cxx" | "c++" => Some(LanguagePattern::Cpp),
            "c" | "h" => Some(LanguagePattern::C),
            "cs" => Some(LanguagePattern::CSharp),
            "go" => Some(LanguagePattern::Go),
            "rb" => Some(LanguagePattern::Ruby),
            "php" => Some(LanguagePattern::Php),
            "kt" => Some(LanguagePattern::Kotlin),
            "swift" => Some(LanguagePattern::Swift),
            "dart" => Some(LanguagePattern::Dart),
            "scala" => Some(LanguagePattern::Scala),
            _ => None,
        }
    } else {
        None
    }
}

fn extract_and_replace_variables(content: &str, language: &LanguagePattern) -> (String, HashMap<String, String>) {
    let patterns = language.get_variable_patterns();
    let mut replacements: HashMap<String, String> = HashMap::new();
    let mut variation_counter = 0;
    
    // First pass: identify all bad variables
    for pattern in &patterns {
        for cap in pattern.captures_iter(content) {
            // Try to get the variable name from different capture groups
            for i in 1..cap.len() {
                if let Some(var_match) = cap.get(i) {
                    let var_name = var_match.as_str();
                    if is_bad_variable(var_name) && !replacements.contains_key(var_name) {
                        let replacement = get_yourmom_variation(variation_counter);
                        replacements.insert(var_name.to_string(), replacement);
                        variation_counter += 1;
                    }
                }
            }
        }
    }
    
    // Second pass: replace all occurrences
    let mut result = content.to_string();
    for (old_name, new_name) in &replacements {
        // Create a regex that matches the variable name with word boundaries
        // But be careful with languages that use different identifier rules
        let boundary_pattern = match language {
            LanguagePattern::JavaScript | LanguagePattern::Python | LanguagePattern::Ruby => {
                format!(r"\b{}\b", regex::escape(old_name))
            }
            _ => {
                format!(r"\b{}\b", regex::escape(old_name))
            }
        };
        
        if let Ok(re) = Regex::new(&boundary_pattern) {
            result = re.replace_all(&result, new_name.as_str()).to_string();
        }
    }
    
    (result, replacements)
}

fn process_file(file_path: &Path, in_place: bool, backup: bool) -> io::Result<()> {
    let language = detect_language(file_path);
    
    if language.is_none() {
        println!("Skipping {}: unsupported file type", file_path.display());
        return Ok(());
    }
    
    let lang = language.unwrap();
    println!("Processing {} ({:?})", file_path.display(), lang);
    
    let content = modifyfile::read_file(file_path.to_str().unwrap())?;
    let (modified_content, replacements) = extract_and_replace_variables(&content, &lang);
    
    if replacements.is_empty() {
        println!("  No bad variables found.");
        return Ok(());
    }
    
    println!("  Replaced variables:");
    for (old, new) in &replacements {
        println!("    {} -> {}", old, new);
    }
    
    if backup && in_place {
        let backup_path = format!("{}.backup", file_path.display());
        fs::copy(file_path, &backup_path)?;
        println!("  Backup created: {}", backup_path);
    }
    
    if in_place {
        modifyfile::write_file(file_path.to_str().unwrap(), &modified_content)?;
        println!("  File modified in place.");
    } else {
        let output_file = format!("{}.fixed", file_path.display());
        modifyfile::write_file(&output_file, &modified_content)?;
        println!("  Modified content written to: {}", output_file);
    }
    
    Ok(())
}

fn find_source_files(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if detect_language(&path).is_some() {
                    files.push(path);
                }
            } else if path.is_dir() && recursive && !path.file_name().unwrap_or_default().to_str().unwrap_or("").starts_with('.') {
                files.extend(find_source_files(&path, recursive));
            }
        }
    }
    
    files
}

fn print_help() {
    println!("YourMom Variable Fixer - Universal bad variable name replacer");
    println!();
    println!("Supports: Rust, JavaScript/TypeScript, Python, Java, C/C++, C#, Go, Ruby, PHP, Kotlin, Swift, Dart, Scala");
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS] <files_or_directories>", env::args().nth(0).unwrap_or("yourmom-fixer".to_string()));
    println!();
    println!("OPTIONS:");
    println!("    -i, --in-place      Modify files in place instead of creating .fixed files");
    println!("    -r, --recursive     Process directories recursively");
    println!("    -b, --backup        Create .backup files when using --in-place");
    println!("    --dry-run           Show what would be changed without modifying files");
    println!("    -h, --help          Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    {} main.rs lib.py                    # Process specific files");
    println!("    {} -i -b src/                        # Process src/ in-place with backups");
    println!("    {} -r .                              # Process all source files recursively");
    println!("    {} --dry-run -r ./project            # Preview changes without modifying");
    println!();
    println!("SUPPORTED EXTENSIONS:");
    println!("    .rs .js .jsx .ts .tsx .py .java .cpp .cc .cxx .c++ .c .h .cs .go .rb .php .kt .swift .dart .scala");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return Ok(());
    }
    
    let mut in_place = false;
    let mut recursive = false;
    let mut backup = false;
    let mut dry_run = false;
    let mut paths = Vec::new();
    
    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-i" | "--in-place" => in_place = true,
            "-r" | "--recursive" => recursive = true,
            "-b" | "--backup" => backup = true,
            "--dry-run" => dry_run = true,
            arg => {
                if arg.starts_with('-') {
                    eprintln!("Unknown option: {}", arg);
                    return Ok(());
                }
                paths.push(arg);
            }
        }
        i += 1;
    }
    
    if paths.is_empty() {
        eprintln!("Error: No input files or directories specified.");
        print_help();
        return Ok(());
    }
    
    if dry_run {
        println!("DRY RUN MODE - No files will be modified\n");
    }
    
    let mut all_files = Vec::new();
    
    // Collect all files to process
    for path_str in paths {
        let path = Path::new(path_str);
        if path.is_file() {
            all_files.push(path.to_path_buf());
        } else if path.is_dir() {
            all_files.extend(find_source_files(path, recursive));
        } else {
            eprintln!("Warning: {} does not exist", path.display());
        }
    }
    
    if all_files.is_empty() {
        println!("No supported source files found.");
        return Ok(());
    }
    
    println!("Found {} files to process\n", all_files.len());
    
    for file_path in all_files {
        if dry_run {
            // For dry run, just show what would be replaced
            if let Some(lang) = detect_language(&file_path) {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    let (_, replacements) = extract_and_replace_variables(&content, &lang);
                    if !replacements.is_empty() {
                        println!("Would process {} ({:?}):", file_path.display(), lang);
                        for (old, new) in replacements {
                            println!("  {} -> {}", old, new);
                        }
                    }
                }
            }
        } else {
            match process_file(&file_path, in_place, backup) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Error processing {}: {}", file_path.display(), e);
                }
            }
        }
        println!();
    }
    
    if dry_run {
        println!("Dry run complete. Use without --dry-run to apply changes.");
    } else {
        println!("Processing complete!");
    }
    
    Ok(())
}
