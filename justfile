# Stop a recipe on a failed command, an unset variable or a failed pipe
set shell := ["bash", "-euo", "pipefail", "-c"]

nargo := env("NARGO", "nargo")
bb := env("BB", "bb")

# Default recipe
default:
    @just --list

# --- All components ---

# Run every check that CI runs
all-check: contracts-check

# --- Contracts ---

# Install contract dependencies
contracts-deps:
    cd contracts && forge soldeer install

# Install the contract tooling from the lockfile
contracts-tooling:
    cd contracts && bun install --frozen-lockfile

# Clean contracts
contracts-clean:
    cd contracts && forge clean

# Build contracts
contracts-build *args:
    cd contracts && forge build {{ args }}

# Run contract tests
contracts-test *args:
    cd contracts && forge test --force {{ args }}

# Lint contracts: forge lint, then solhint for the rules that forge lint lacks
contracts-lint:
    cd contracts && forge lint --deny notes
    cd contracts && bunx --bun solhint --config .solhint.json 'src/**/*.sol'
    cd contracts && bunx --bun solhint --config .solhint.other.json 'test/**/*.sol'

# Run slither on contracts
contracts-static-analysis:
    cd contracts && slither .
    @echo "Removing slither compilation artifacts..."
    cd contracts && forge clean

# Format contracts
contracts-fmt *args:
    cd contracts && forge fmt {{ args }}

# Check contract formatting
contracts-fmt-check:
    cd contracts && forge fmt --check

# Run every contract check that CI runs
contracts-check: contracts-fmt-check contracts-lint contracts-static-analysis contracts-clean contracts-build contracts-test
