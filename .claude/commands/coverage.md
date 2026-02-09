# /coverage - Check test coverage

```bash
cargo tarpaulin --skip-clean 2>&1 | tail -20
```

Add tests for any uncovered lines.
