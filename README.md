# Deva MCP Server

Model Context Protocol Server para integração com AI agents.

## Instalação

```bash
cargo build --release
./target/release/deva-mcp --help
```

## Uso

```bash
# Server standalone
./target/release/deva-mcp

# Com features
./target/release/deva-mcp --features github,azure_devops
```

## Configuração Claude Desktop

```json
{
  "mcpServers": {
    "deva": {
      "command": "./target/release/deva-mcp"
    }
  }
}
```

## Desenvolvimento

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace
```

## VSCode

Abrir o workspace deva-workspace.code-workspace na raiz do projeto.
