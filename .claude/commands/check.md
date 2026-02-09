# /check - Validate before finishing

## Steps

1. **Format:**
   ```bash
   cargo fmt
   ```

2. **Clippy (zero warnings):**
   ```bash
   cargo clippy 2>&1
   ```

3. **Tests:**
   ```bash
   cargo test 2>&1
   ```

4. **Coverage:**
   ```bash
   cargo tarpaulin --skip-clean 2>&1 | tail -5
   ```

5. **Doc tests:**
   ```bash
   cargo test --doc 2>&1
   ```

6. Report results.
