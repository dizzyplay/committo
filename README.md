# Committo

Commit-message generator

## Install (Build from source)

```bash
# Build
cargo build --release
````

## Install (Homebrew)

```bash
brew tap dizzyplay/committo
brew install committo

brew update
brew upgrade committo
```

## Usage

### Configuration

```bash
# Set values
committo set api-key your-key-here
committo set candidate-count 5
committo set llm-model gpt-4

# Show current settings
committo show
```

### Generate commit messages

```bash
git add .
committo                # or: committo generate   – actually calls the API
committo generate --dry-run   # Dry-run (shows prompt only, no API call)
```

## Convention file

Define hierarchical commit rules with a `.committoconvention` file:

```bash
# Home directory – personal preferences
echo "Prefer concise and clear Korean commit messages" > ~/.committoconvention

# Project root – project-wide rules
echo "Conventional Commits format: feat/fix/docs/refactor" > /project/.committoconvention

# Monorepo package – detailed conventions
echo "frontend: Use 'component:' prefix for UI component changes" > /project/frontend/.committoconvention
```

**Prompt merge order:** parent → child directories, so messages become increasingly specific and context-aware.

## First-time setup

If no config file is found, an interactive setup runs automatically:

```bash
$ committo
No configuration file found at: /Users/user/committo.toml
...
```

## Example

```bash
$ git add src/lib.rs
$ committo
🔄 Retry (generate new messages)
feat: Add regex validation for env-var parsing
refactor: Centralize config-file loading logic
Select a commit message: feat: Add regex validation for env-var parsing
```


