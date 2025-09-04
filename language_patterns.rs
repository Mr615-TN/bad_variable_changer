use regex::Regex;

#[derive(Debug, Clone)]
pub enum LanguagePattern {
    Rust,
    JavaScript,
    Python,
    Java,
    Cpp,
    C,
    CSharp,
    Go,
    Ruby,
    Php,
    Kotlin,
    Swift,
    Dart,
    Scala,
}

impl LanguagePattern {
    pub fn get_variable_patterns(&self) -> Vec<Regex> {
        match self {
            LanguagePattern::Rust => vec![
                // let var_name =, let mut var_name =
                Regex::new(r"\blet\s+(?:mut\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*[=:]").unwrap(),
                // for var_name in
                Regex::new(r"\bfor\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+in\b").unwrap(),
                // function parameters: fn name(var_name: type)
                Regex::new(r"\bfn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*:").unwrap(),
                // closure parameters: |var_name|
                Regex::new(r"\|([a-zA-Z_][a-zA-Z0-9_]*)\|").unwrap(),
                // match patterns
                Regex::new(r"\bSome\(([a-zA-Z_][a-zA-Z0-9_]*)\)").unwrap(),
                Regex::new(r"\bOk\(([a-zA-Z_][a-zA-Z0-9_]*)\)").unwrap(),
                Regex::new(r"\bErr\(([a-zA-Z_][a-zA-Z0-9_]*)\)").unwrap(),
            ],
            
            LanguagePattern::JavaScript => vec![
                // let/const/var declarations
                Regex::new(r"\b(?:let|const|var)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*[=;]").unwrap(),
                // function parameters
                Regex::new(r"\bfunction\s+[a-zA-Z_$][a-zA-Z0-9_$]*\s*\([^)]*\b([a-zA-Z_$][a-zA-Z0-9_$]*)\s*[,)]").unwrap(),
                // arrow function parameters
                Regex::new(r"\(([a-zA-Z_$][a-zA-Z0-9_$]*)\)\s*=>").unwrap(),
                Regex::new(r"\b([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=>").unwrap(),
                // for loops
                Regex::new(r"\bfor\s*\(\s*(?:let|const|var)?\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s+(?:in|of)\b").unwrap(),
                // destructuring
                Regex::new(r"\{\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\}").unwrap(),
                Regex::new(r"\[\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\]").unwrap(),
            ],
            
            LanguagePattern::Python => vec![
                // variable assignment
                Regex::new(r"^(\s*)([a-zA-Z_][a-zA-Z0-9_]*)\s*=").unwrap(),
                // for loops
                Regex::new(r"\bfor\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+in\b").unwrap(),
                // function parameters
                Regex::new(r"\bdef\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
                // lambda parameters
                Regex::new(r"\blambda\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:").unwrap(),
                // with statements
                Regex::new(r"\bwith\s+[^)]+\s+as\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:").unwrap(),
                // except clauses
                Regex::new(r"\bexcept\s+\w+\s+as\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:").unwrap(),
            ],
            
            LanguagePattern::Java => vec![
                // variable declarations
                Regex::new(r"\b(?:int|long|short|byte|float|double|boolean|char|String|var)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=;]").unwrap(),
                // for loops
                Regex::new(r"\bfor\s*\([^;]*;\s*[^;]*;\s*[^)]*\)\s*\{").unwrap(),
                // enhanced for loops
                Regex::new(r"\bfor\s*\(\s*\w+\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*[^)]+\)").unwrap(),
                // method parameters
                Regex::new(r"\b(?:public|private|protected|static)?\s*\w+\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
                // catch blocks
                Regex::new(r"\bcatch\s*\(\s*\w+\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\)").unwrap(),
            ],
            
            LanguagePattern::Cpp | LanguagePattern::C => vec![
                // variable declarations
                Regex::new(r"\b(?:int|long|short|char|float|double|bool|auto|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=;,)]").unwrap(),
                // for loops
                Regex::new(r"\bfor\s*\([^;]*\b(?:int|auto)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=;]").unwrap(),
                // function parameters
                Regex::new(r"\b\w+\s+\w+\s*\([^)]*\b\w+\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
            ],
            
            LanguagePattern::CSharp => vec![
                // variable declarations
                Regex::new(r"\b(?:int|long|short|byte|float|double|bool|string|var)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=;]").unwrap(),
                // foreach loops
                Regex::new(r"\bforeach\s*\(\s*\w+\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+in\s+[^)]+\)").unwrap(),
                // method parameters
                Regex::new(r"\b(?:public|private|protected|internal)?\s*\w+\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
            ],
            
            LanguagePattern::Go => vec![
                // variable declarations
                Regex::new(r"\b(?:var\s+([a-zA-Z_][a-zA-Z0-9_]*)|([a-zA-Z_][a-zA-Z0-9_]*)\s*:=)").unwrap(),
                // for loops
                Regex::new(r"\bfor\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(?::=|,)").unwrap(),
                // function parameters
                Regex::new(r"\bfunc\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s+\w+\s*[,)]").unwrap(),
                // range loops
                Regex::new(r"\bfor\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:,\s*[a-zA-Z_][a-zA-Z0-9_]*)?\s*:=\s*range").unwrap(),
            ],
            
            LanguagePattern::Ruby => vec![
                // variable assignment
                Regex::new(r"^(\s*)([a-zA-Z_][a-zA-Z0-9_]*)\s*=").unwrap(),
                // block parameters
                Regex::new(r"\bdo\s*\|\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\|").unwrap(),
                Regex::new(r"\{\s*\|\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\|").unwrap(),
                // method parameters
                Regex::new(r"\bdef\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
            ],
            
            LanguagePattern::Php => vec![
                // variable assignment (PHP variables start with $)
                Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_]*)\s*=").unwrap(),
                // function parameters
                Regex::new(r"\bfunction\s+\w+\s*\([^)]*\$([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
                // foreach loops
                Regex::new(r"\bforeach\s*\([^)]+\s+as\s+\$([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
            ],
            
            LanguagePattern::Kotlin => vec![
                // variable declarations
                Regex::new(r"\b(?:val|var)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=:]").unwrap(),
                // for loops
                Regex::new(r"\bfor\s*\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s+in\s+[^)]+\)").unwrap(),
                // function parameters
                Regex::new(r"\bfun\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*\w+\s*[,)]").unwrap(),
            ],
            
            LanguagePattern::Swift => vec![
                // variable declarations
                Regex::new(r"\b(?:let|var)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=:]").unwrap(),
                // for loops
                Regex::new(r"\bfor\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+in\s+").unwrap(),
                // function parameters
                Regex::new(r"\bfunc\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*\w+\s*[,)]").unwrap(),
            ],
            
            LanguagePattern::Dart => vec![
                // variable declarations
                Regex::new(r"\b(?:var|final|const|int|double|String|bool)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=;]").unwrap(),
                // for loops
                Regex::new(r"\bfor\s*\(\s*\w+\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+in\s+[^)]+\)").unwrap(),
                // function parameters
                Regex::new(r"\b\w+\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*[,)]").unwrap(),
            ],
            
            LanguagePattern::Scala => vec![
                // variable declarations
                Regex::new(r"\b(?:val|var)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[=:]").unwrap(),
                // for comprehensions
                Regex::new(r"\bfor\s*\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*<-").unwrap(),
                // function parameters
                Regex::new(r"\bdef\s+\w+\s*\([^)]*\b([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*\w+\s*[,)]").unwrap(),
            ],
        }
    }
}
