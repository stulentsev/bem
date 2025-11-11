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

#### `bem get :id`
**Status**: ✅ Implemented  
**Purpose**: Universal command to retrieve any resource by ID (auto-detects type from prefix)  
**Input**: Resource ID (positional argument)  
**Output**: Pretty-printed JSON to stdout  
**Supported Prefixes**:
- `evt_` - Routes to events API
- `tr_` - Routes to transformations API

**Example Usage**:
```bash
export BEM_API_TOKEN="your_api_key"
bem get evt_abc123
bem get tr_2bxoJPNdSD4LgRT4YVC4gt72hlI
```

**Notes**:
- This is a convenience command that automatically routes to the appropriate endpoint based on ID prefix
- Returns an error if the ID doesn't start with a recognized prefix
- The specific `bem events get` and `bem transformations get` commands are still available

#### `bem events get :id`
**Status**: ✅ Implemented  
**Purpose**: Retrieve event details by ID from bem.ai API  
**Input**: Event ID (positional argument)  
**Output**: Pretty-printed JSON to stdout  
**API Endpoint**: `GET https://api.bem.ai/v1-alpha/events/:id`

**Example Usage**:
```bash
export BEM_API_TOKEN="your_api_key"
bem events get abc123
```

**Expected Output**:
```json
{
  "eventID": "abc123",
  "status": "completed",
  ...
}
```

#### `bem transformations get :id`
**Status**: ✅ Implemented  
**Purpose**: Retrieve transformation details by ID from bem.ai API  
**Input**: Transformation ID (positional argument)  
**Output**: Pretty-printed JSON to stdout  
**API Endpoint**: `GET https://api.bem.ai/v1-beta/transformations?transformationIDs=:id`

**Example Usage**:
```bash
export BEM_API_TOKEN="your_api_key"
bem transformations get tr_2bxoJPNdSD4LgRT4YVC4gt72hlI
```

**Expected Output**:
```json
{
  "transformations": [
    {
      "transformationID": "tr_2bxoJPNdSD4LgRT4YVC4gt72hlI",
      ...
    }
  ]
}
```

#### `bem eval :transformation_id`
**Status**: ✅ Implemented  
**Purpose**: Retrieve evaluation results for a transformation  
**Input**: Transformation ID (positional argument)  
**Output**: Pretty-printed JSON to stdout (extracts only the specific transformation's result)  
**API Endpoint**: `GET https://api.bem.ai/v1-beta/transformations/eval/results?transformationIDs=:id`

**Example Usage**:
```bash
export BEM_API_TOKEN="your_api_key"
bem eval tr_2bxoJPNdSD4LgRT4YVC4gt72hlI
```

**Expected Output**:
```json
{
  "fieldMetrics": {
    "/companyName": {
      "confidenceScore": 0.95,
      "reasoning": "Company name is clearly stated in the document header",
      "hallucination": false,
      "relevanceScore": 1
    },
    "/totalAmount": {
      "confidenceScore": 0.88,
      "reasoning": "Amount is extracted from the invoice total line",
      "hallucination": false,
      "relevanceScore": 0.95
    }
  },
  "overallConfidence": 0.935,
  "runtime": 1.23,
  "hasHallucinations": false,
  "evaluationVersion": "0.1.0-gemini",
  "createdAt": "2024-01-15T10:30:00Z"
}
```

**Notes**:
- Automatically extracts the specific transformation's result from the API's multi-object response
- Returns an error if the evaluation is pending or has failed
- Currently supports single transformation only (API supports multiple)

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

