# 006-fix-windows-pdf-filling

Fix three critical issues preventing proper PDF character sheet generation on Windows.

## Problems

1. **Checkboxes not visible** - Proficiency and spell preparation checkboxes don't appear checked in the PDF
2. **Font too large** - Text appears oversized in PDF fields, making content difficult to read
3. **Debug log missing** - Hardcoded path prevents log file creation on non-developer systems

## Solution

- **Checkboxes**: Use PDF Name objects instead of String objects, add `/AS` appearance state property
- **Fonts**: Set `/DA` (Default Appearance) with appropriate font sizes (8-12pt based on field type)
- **Logging**: Replace hardcoded path with cross-platform dynamic path resolution

## Quick Start

```bash
# Review the implementation guide
cat IMPLEMENTATION_GUIDE.md

# Create feature branch
git checkout -b 006-fix-windows-pdf-filling

# Make changes to:
# - src/mcp_server.rs (lines 11-24)
# - src/pdf_filler.rs (lines 113-140, 169-190)

# Test
cargo test
cargo run test
```

## Documentation

| File | Purpose | Lines |
|------|---------|-------|
| **IMPLEMENTATION_GUIDE.md** | 👉 **START HERE** - Step-by-step code changes | 466 |
| spec.md | Complete feature specification | 196 |
| research.md | Technical analysis and PDF spec details | 221 |
| plan.md | 5-phase implementation plan | 286 |
| data-model.md | PDF object model changes | 275 |
| quickstart.md | Quick reference guide | 126 |
| tasks.md | Detailed task checklist | 149 |
| checklists/requirements.md | Requirements verification | 142 |

**Total:** 1,861 lines of documentation

## Files to Modify

- `src/mcp_server.rs` - Update `log_to_file()` function (lines 11-24)
- `src/pdf_filler.rs` - Update `fill_pdf_fields()` method (lines 113-140)
- `src/pdf_filler.rs` - Update `mark_checkboxes()` method (lines 169-190)

**Total changes:** ~50 lines + 2 new helper methods

## Estimated Time

- Phase 1 (Logging): 30 minutes ⚡ LOW RISK
- Phase 2 (Font Size): 1-2 hours ⚠️ MEDIUM RISK
- Phase 3 (Checkboxes): 2-3 hours 🔴 HIGH RISK
- Testing & Documentation: 1.5 hours

**Total: 5-7 hours**

## Testing Checklist

- [ ] Debug log created in `./mcp_debug.log`
- [ ] Character name at 12pt font
- [ ] Spell fields at 8pt font
- [ ] Checkboxes visible in Adobe Reader (Windows)
- [ ] Checkboxes visible in Chrome PDF viewer
- [ ] Checkboxes visible in Edge PDF viewer
- [ ] No regression on macOS/Linux
- [ ] All tests pass: `cargo test`

## Key Insights

- PDF button fields require Name objects per PDF 1.7 spec
- `/AS` (Appearance State) must match `/V` (Value) for checkboxes
- `/DA` format: `"/Helv 10 Tf 0 g"` (font name, size, color)
- Windows PDF viewers enforce PDF spec more strictly than macOS viewers

## Dependencies

No new dependencies required. Uses:
- `lopdf` (existing)
- `std::env` (standard library)
- `std::fs` (standard library)

## Status

- **Created:** 2025-01-20
- **Status:** Ready for implementation
- **Priority:** P0 (Windows compatibility critical)
- **Complexity:** Medium-High
