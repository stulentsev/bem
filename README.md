# bem

A command-line interface for interacting with the [bem.ai](https://bem.ai) API - a SaaS platform for extracting structured data from PDFs and images.

## Installation

### From Source

```bash
git clone https://github.com/stulentsev/bem.git
cd bem
cargo build --release
```

The binary will be available at `target/release/bem`.

To install globally:

```bash
cargo install --path .
```

## Configuration

The CLI requires a bem.ai API token for authentication. You can provide it in two ways:

### Option 1: Environment Variable

```bash
export BEM_API_TOKEN="your_api_token_here"
```

### Option 2: Configuration File

Create a `~/.bemrc` file in your home directory:

```bash
echo "BEM_API_TOKEN=your_api_token_here" > ~/.bemrc
```

To obtain an API token, visit the [bem.ai documentation](https://docs.bem.ai).

## Usage

### Universal Get Command

The `get` command automatically detects the resource type from the ID prefix:

```bash
# Get an event (IDs starting with evt_)
bem get evt_abc123

# Get a transformation (IDs starting with tr_)
bem get tr_2bxoJPNdSD4LgRT4YVC4gt72hlI
```

### Events

Retrieve event details by ID:

```bash
bem events get evt_abc123
```

**Example output:**
```json
{
  "eventID": "evt_abc123",
  "status": "completed",
  ...
}
```

### Transformations

Retrieve transformation details by ID:

```bash
bem transformations get tr_2bxoJPNdSD4LgRT4YVC4gt72hlI
```

**Example output:**
```json
{
  "transformationID": "tr_2bxoJPNdSD4LgRT4YVC4gt72hlI",
  ...
}
```

## API Endpoints

- **Events**: `https://api.bem.ai/v1-alpha/events/:id`
- **Transformations**: `https://api.bem.ai/v1-beta/transformations?transformationIDs=:id`

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running in Development

```bash
cargo run -- get evt_abc123
```

## Dependencies

- `clap` - Command-line argument parsing
- `reqwest` - HTTP client for API calls
- `serde` / `serde_json` - JSON serialization/deserialization
- `tokio` - Async runtime
- `anyhow` - Error handling
- `dotenvy` - Environment variable loading

## Documentation

For more information about the bem.ai API and available features, visit:
- [bem.ai Documentation](https://docs.bem.ai)
- [bem.ai Website](https://bem.ai)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Your chosen license here]

## Support

For issues related to:
- **This CLI tool**: Open an issue on GitHub
- **The bem.ai API**: Contact [bem.ai support](https://docs.bem.ai)

