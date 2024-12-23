# linelint

`linelint` is a tool designed to lint and automatically format files by enforcing consistent line formatting rules.

## Lint Rules

### LineEndLint
Ensures files end with a newline (`\n`).

**Fix:** Add a newline at the end.

### TrailingWhitespaceLint
Flags trailing spaces at the end of lines.

**Fix:** Remove trailing spaces.

## Cli Usage Example(exclude dir or file) 
```bash
./cli check --exclude 'linelint/src,linelint-cli/src/main.rs'

./cli format --exclude 'linelint/src,linelint-cli/src/main.rs'

./cli check -e 'linelint/src/rule,linelint-cli'

./cli format -e 'linelint/src/rule,linelint-cli'
```
