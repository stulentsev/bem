# AGENTS.md

## Project Overview

This is a Rust CLI application for interacting with bem.ai, a SaaS platform for extracting structured data from PDFs and images.

**Vendor Documentation**: https://docs.bem.ai

## Architecture

### Technology Stack
- **Language**: Rust
- **Type**: Command-line interface (CLI)

### Project Structure
```
bem/
├── Cargo.toml          # Rust dependencies and project metadata
├── src/
│   └── main.rs         # CLI entry point
└── AGENTS.md           # This file
```

## Security & Configuration

### API Authentication
- **NEVER** hardcode API tokens in source code
- Use environment variables for sensitive configuration
- API token must be read from `BEM_API_TOKEN` environment variable
- Use Rust's standard library (`std::env`) to fetch environment variables

Example:
```rust
let api_token = std::env::var("BEM_API_TOKEN")
    .expect("BEM_API_TOKEN environment variable must be set");
```

## Commands & Features

### Current Implementation Status

#### `bem events get :id`
**Status**: To be implemented  
**Purpose**: Retrieve event details by ID from bem.ai API  
**Input**: Event ID (positional argument)  
**Output**: Pretty-printed JSON to stdout

**Requirements**:
- Accept event ID as a command-line argument
- Make API request to bem.ai events endpoint
- Output response as formatted JSON
- Handle errors gracefully

**Example Usage**:
```bash
bem events get abc123
```

**Expected Output**:
```json
{
  "id": "abc123",
  "status": "completed",
  ...
}
```

## Development Guidelines

### Code Style
- Follow standard Rust conventions and idioms
- Use proper error handling (Result types, proper error propagation)
- Keep CLI code modular and testable

### Dependencies
Consider these common Rust CLI crates:
- `clap` - Command-line argument parsing
- `reqwest` - HTTP client for API calls
- `serde` / `serde_json` - JSON serialization/deserialization
- `tokio` - Async runtime (if using async/await)
- `dotenv` - Environment variable loading (optional)
- `anyhow` or `thiserror` - Error handling

### API Integration
- Base URL and endpoint structure should be derived from bem.ai documentation
- Include proper headers (Authorization, Content-Type)
- Handle HTTP errors and network failures
- Parse JSON responses using serde

### Testing
- Unit tests for core logic
- Integration tests for CLI commands
- Mock API responses where appropriate

## Future Enhancements

As the CLI grows, consider:
- Additional commands for other bem.ai resources
- Configuration file support (beyond environment variables)
- Output format options (JSON, YAML, table)
- Verbose/debug logging modes
- Pagination for list commands
- Batch operations

## Notes for AI Assistants

When implementing features:
1. **Always** check vendor documentation for API specifications
2. **Never** commit API tokens or sensitive data
3. Prefer minimal, focused changes to existing code
4. Maintain consistency with Rust CLI best practices
5. Add inline comments for complex business logic
6. Update this file when architecture changes significantly

