# Pygmalion

A high-performance, developer-centric, and extensible command-line interface (CLI) tool for automatically generating infrastructure diagrams from Infrastructure as Code (IaC) definitions.

> 📖 **日本語版ドキュメント**: [docs/ja.md](docs/ja.md) で日本語版のドキュメントをご覧いただけます。

## 🎯 Mission

Pygmalion provides a unified visualization solution for infrastructure by converting any IaC definition into any diagram format through a robust common intermediate representation (IR). Our mission is to solve the fragmentation in modern DevOps environments where teams use multiple IaC tools (Terraform, AWS CDK, Pulumi) but lack unified visualization and documentation tools.

## 🏗️ Architecture

Pygmalion follows a three-stage pipeline architecture with a plugin-based ecosystem:

```
Parse → Intermediate Representation → Render
```

### Core Components

1. **Parse Stage**: IaC-specific parser plugins convert source code into standardized IR JSON objects
2. **Intermediate Representation (IR)**: A declarative, serializable JSON format that represents the complete infrastructure graph
3. **Render Stage**: Renderer plugins convert IR into visual outputs (Draw.io, Mermaid.js, etc.)

### Plugin Ecosystem

- **Language Agnostic**: Plugins are standalone executables discovered via PATH
- **Standard IPC**: Communication via stdin/stdout with JSON
- **Extensible**: Support for any IaC tool or diagram format through plugins

## 🚀 Features

### Current (MVP)
- **Multi-IaC Support**: Terraform, AWS CDK, Pulumi
- **Multi-Format Output**: Draw.io, Mermaid.js
- **Plugin Architecture**: Extensible parser and renderer system
- **High Performance**: Rust-based implementation
- **CI/CD Ready**: Single native binary distribution

### Planned
- **Cost Analysis**: Integration with cloud pricing APIs
- **Operational Simulation**: Generate usage examples and scripts
- **Documentation Generation**: Auto-generate comprehensive README files
- **Security Scanning**: Visualize compliance and security findings

## 📦 Installation

### Prerequisites
- Rust 1.70+ (for development)
- Terraform, AWS CDK, or Pulumi projects to visualize

### Quick Start
```bash
# Clone the repository
git clone https://github.com/your-org/pygmalion.git
cd pygmalion

# Build from source
cargo build --release

# Install globally
cargo install --path .
```

## 🛠️ Usage

### Basic Commands

```bash
# Generate diagram from Terraform project
pygma generate ./terraform-project --from terraform --to drawio

# Generate Mermaid diagram from CDK project
pygma generate ./cdk-project --from cdk --to mermaid -o architecture.md

# Parse only (output IR JSON)
pygma parse ./terraform-project --from terraform > infra.ir.json

# Render only (from IR JSON)
pygma render infra.ir.json --to drawio -o diagram.drawio.xml

# List available plugins
pygma plugin list
```

### Command Reference

| Command | Arguments | Description |
|---------|-----------|-------------|
| `generate <PATH>` | `--from <FORMAT>`, `--to <FORMAT>`, `-o <FILE>` | Parse and render in one command |
| `parse <PATH>` | `--from <FORMAT>`, `-o <FILE>` | Parse IaC to IR JSON |
| `render <IR_FILE>` | `--to <FORMAT>`, `-o <FILE>` | Render IR to diagram format |
| `plugin list` | None | List discovered plugins |

### Supported Formats

**Input IaC Formats:**
- `terraform` - Terraform HCL files
- `cdk` - AWS CDK projects (TypeScript, Python, etc.)
- `pulumi` - Pulumi projects

**Output Diagram Formats:**
- `drawio` - Draw.io compatible XML
- `mermaid` - Mermaid.js syntax

## 🔌 Plugin Development

Pygmalion's plugin architecture allows you to extend support for any IaC tool or diagram format.

### Plugin Discovery
Plugins are standalone executables following the naming convention:
- Parsers: `pygma-parser-<format>`
- Renderers: `pygma-renderer-<format>`

### Plugin Interface

**Parser Plugin Contract:**
```bash
# Input: Command line arguments
pygma-parser-terraform --input-path ./project --output-path ./output

# Output: IR JSON to stdout
{
  "irVersion": "1.0.0",
  "nodes": [...],
  "edges": [...],
  "metadata": {...}
}
```

**Renderer Plugin Contract:**
```bash
# Input: IR JSON from stdin
cat infra.ir.json | pygma-renderer-drawio --output diagram.drawio.xml

# Output: Diagram file to specified path or stdout
```

### IR Schema

The Intermediate Representation (IR) is a standardized JSON format:

```json
{
  "irVersion": "1.0.0",
  "metadata": {
    "sourceType": "terraform"
  },
  "nodes": [
    {
      "id": "vpc-123",
      "type": "Resource",
      "label": "Primary VPC",
      "properties": {
        "instance_type": "t3.micro"
      },
      "metadata": {
        "cost": {
          "monthly": 7.59
        }
      },
      "parent": "subnet-abc",
      "icon": "aws-ec2-instance"
    }
  ],
  "edges": [
    {
      "id": "dep-vpc-subnet",
      "source": "vpc-123",
      "target": "subnet-abc",
      "type": "Dependency",
      "label": "depends on"
    }
  ]
}
```

## 🏛️ Architecture Details

### Three-Stage Pipeline

1. **Parse Stage**: IaC-specific parsers extract infrastructure information
2. **IR Stage**: Unified graph model with rich metadata
3. **Render Stage**: Layout engines and visual formatting

### Layout Engine

Pygmalion uses the Sugiyama method (hierarchical graph drawing) for optimal diagram layout:
- Minimizes edge crossings
- Creates clear dependency flows
- Supports logical grouping and clustering

### Performance Optimizations

- **Rust Implementation**: Memory safety and high performance
- **Single Binary**: Easy distribution and CI/CD integration
- **Streaming Processing**: Handles large infrastructure graphs efficiently
- **Caching**: IR caching for incremental updates

## 🛣️ Roadmap

### Phase 1: Core Platform (Current)
- [x] Basic CLI framework
- [ ] IR schema implementation
- [ ] Plugin system
- [ ] Terraform parser
- [ ] Draw.io renderer
- [ ] Mermaid renderer

### Phase 2: Enhanced Features
- [ ] Cost analysis engine
- [ ] Operational simulation
- [ ] Documentation generation
- [ ] Security scanning integration

### Phase 3: Advanced Capabilities
- [ ] Real-time monitoring integration
- [ ] Multi-cloud support
- [ ] Advanced layout algorithms
- [ ] Interactive web interface

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone and setup
git clone https://github.com/your-org/pygmalion.git
cd pygmalion

# Install dependencies
cargo build

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Plugin Development

1. Create a new executable following the naming convention
2. Implement the plugin contract (stdin/stdout JSON communication)
3. Add tests and documentation
4. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [cdk-graph](https://github.com/awslabs/aws-pdk/tree/mainline/ws/cdk-graph) from AWS PDK
- Built with [clap](https://github.com/clap-rs/clap) for CLI framework
- Uses [Graphviz](https://graphviz.org/) for layout algorithms
- Icons from [AWS Architecture Icons](https://aws.amazon.com/architecture/icons/)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-org/pygmalion/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/pygmalion/discussions)
- **Documentation**: [Wiki](https://github.com/your-org/pygmalion/wiki)

---

**Pygmalion**: Transforming Infrastructure as Code into beautiful, meaningful diagrams. 
