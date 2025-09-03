# 🔥 YourMom Variable Fixer 🔥

A universal command-line tool that finds terrible variable names in your codebase and replaces them with increasingly ridiculous variations of "YourMom". Because if you're going to have bad variable names, they might as well be memorable.

## ✨ Features

- **Multi-language support**: Works with 15+ programming languages
- **Smart detection**: Identifies genuinely bad variable names (single letters, generic names, etc.)
- **Safe replacements**: Uses word boundaries to avoid breaking your code
- **Flexible output**: Create fixed files or modify in-place with optional backups
- **Dry run mode**: Preview changes before applying them
- **Recursive processing**: Handle entire directory trees

## 🎯 Supported Languages

- **Rust** (`.rs`)
- **JavaScript/TypeScript** (`.js`, `.jsx`, `.ts`, `.tsx`)
- **Python** (`.py`)
- **Java** (`.java`)
- **C/C++** (`.c`, `.h`, `.cpp`, `.cc`, `.cxx`, `.c++`)
- **C#** (`.cs`)
- **Go** (`.go`)
- **Ruby** (`.rb`)
- **PHP** (`.php`)
- **Kotlin** (`.kt`)
- **Swift** (`.swift`)
- **Dart** (`.dart`)
- **Scala** (`.scala`)

## 🚀 Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/yourmom-fixer.git
cd yourmom-fixer

# Build the release binary
cargo build --release

# The binary will be in target/release/yourmom-fixer
# Copy it to your PATH for global access
cp target/release/yourmom-fixer /usr/local/bin/
```

### Using Cargo

```bash
cargo install --git https://github.com/yourusername/yourmom-fixer.git
```

## 🎮 Usage

### Basic Usage

```bash
# Process specific files (creates .fixed versions)
yourmom-fixer main.py utils.js

# Process files in-place with backups
yourmom-fixer -i -b src/main.rs src/lib.rs

# Process entire directory recursively
yourmom-fixer -r ./my-project

# Preview changes without modifying files
yourmom-fixer --dry-run -r ./src
```

### Advanced Usage

```bash
# Process multiple directories with different options
yourmom-fixer -i -r -b ./frontend ./backend ./shared

# Process only specific file types in current directory
yourmom-fixer *.py *.js *.rs

# Safe exploration of a new codebase
yourmom-fixer --dry-run -r ./downloaded-project
```

## 🎭 What Gets Replaced

### Bad Variable Names Detected:
- Single letters: `i`, `j`, `k`, `a`, `b`, `c`, etc.
- Generic names: `temp`, `tmp`, `var`, `data`, `item`, `elem`, `node`, `obj`
- Names with numbers: `var1`, `item2`, `data3`
- Really short meaningless names: `aa`, `bb`, `cc`

### YourMom Variations Used:
```
yourmom → yOurMom → YourMom → yourMom → YOURMOM
YouRmOm → yOuRmOm → YoUrMoM → yourmOM → YOURmom
YoUrMoThEr → yourmother → YourMother → YOURMOTHER
yOuRmOtHeR → yourmommy → YourMommy → YOURMOMMY
urmom → UrMom → URMOM → yomama → YoMama → YOMAMA
```

## 📖 Command Line Options

```
OPTIONS:
    -i, --in-place      Modify files in place instead of creating .fixed files
    -r, --recursive     Process directories recursively
    -b, --backup        Create .backup files when using --in-place
    --dry-run           Show what would be changed without modifying files
    -h, --help          Show help message

EXAMPLES:
    yourmom-fixer main.rs lib.py                    # Process specific files
    yourmom-fixer -i -b src/                        # Process src/ in-place with backups
    yourmom-fixer -r .                              # Process all source files recursively
    yourmom-fixer --dry-run -r ./project            # Preview changes without modifying
```

## 🎯 Example Transformations

### Before:
```python
def calculate_something(a, b):
    temp = a * b
    for i in range(10):
        data = temp + i
        if data > 50:
            return data
    return temp
```

### After:
```python
def calculate_something(yourmom, yOurMom):
    YourMom = yourmom * yOurMom
    for yourMom in range(10):
        YOURMOM = YourMom + yourMom
        if YOURMOM > 50:
            return YOURMOM
    return YourMom
```

### JavaScript Before:
```javascript
const items = [1, 2, 3];
let temp = 0;
for (let i = 0; i < items.length; i++) {
    const val = items[i];
    temp += val;
}
```

### JavaScript After:
```javascript
const items = [1, 2, 3];
let yourmom = 0;
for (let yOurMom = 0; yOurMom < items.length; yOurMom++) {
    const YourMom = items[yOurMom];
    yourmom += YourMom;
}
```

## 🛡️ Safety Features

- **Word boundary matching**: Won't partially replace variable names
- **Language-aware**: Uses appropriate patterns for each programming language
- **Backup creation**: Optional backup files when modifying in-place
- **Dry run mode**: Preview changes before applying
- **Skip already fixed**: Won't replace existing "yourmom" variations

## 🤔 Why Use This?

1. **Educational**: Learn to write better variable names by seeing how ridiculous bad ones look
2. **Code review prep**: Make terrible legacy code memorable before refactoring
3. **Team building**: Bring some humor to your debugging sessions
4. **Refactoring aid**: Easily spot which variables need proper names
5. **Because it's hilarious**: Sometimes you need to laugh at your code

## 🎪 Language-Specific Features

The tool understands different language patterns:

- **Python**: Handles list comprehensions, lambda functions, with statements
- **JavaScript**: Supports arrow functions, destructuring, modern syntax
- **Rust**: Recognizes pattern matching, closures, ownership patterns
- **Java**: Handles enhanced for loops, generics, method signatures
- **Go**: Understands Go's unique syntax and conventions
- **And many more!**

## 🚧 Building from Source

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build Steps
```bash
git clone https://github.com/yourusername/yourmom-fixer.git
cd yourmom-fixer
cargo build --release
```

The optimized binary will be at `target/release/yourmom-fixer`.

### Development
```bash
# Run in development mode
cargo run -- --help

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## 🤝 Contributing

Contributions welcome! Areas for improvement:
- Add support for more programming languages
- Improve variable detection patterns
- Add more YourMom variations
- Better handling of edge cases

## 📜 License

MIT License - see LICENSE file for details.

## ⚠️ Disclaimer

This tool is intended for educational and entertainment purposes. While it's designed to be safe, always backup your code before running it on important projects. The authors are not responsible for any code that becomes sentient and starts insulting your mother.

## 🎉 Fun Stats

After processing your codebase, you'll have:
- ✅ More memorable variable names
- ✅ A good laugh during code reviews  
- ✅ Easy identification of variables that need proper names
- ✅ The most unique codebase in your organization

---

*"Your code may be bad, but at least now it's hilariously bad."* - YourMom Variable Fixer