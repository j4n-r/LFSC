# sc-core

## Development

### Caching SQLx Queries

To cache SQLx queries for offline compilation, run:

```bash
cargo sqlx prepare
```

This generates query metadata files in `.sqlx/` that allow the project to compile without a database connection.