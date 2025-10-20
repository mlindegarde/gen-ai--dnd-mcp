# Execution Plan: Fix Windows PDF Filling Issues

**Last Updated:** 2025-01-20  
**Status:** Ready to execute  
**Estimated Total Time:** 5-7 hours

## Quick Start

```bash
# 1. Commit the specification
git add specs/006-fix-windows-pdf-filling/
git commit -m "spec: Add specification for Windows PDF fixing (006)"
git push origin main

# 2. Create feature branch
git checkout -b 006-fix-windows-pdf-filling
git push -u origin 006-fix-windows-pdf-filling

# 3. Follow the phases below
```

---

## Phase Checklist

- [ ] **Step 0:** Commit specification (5 min)
- [ ] **Step 1:** Create feature branch (2 min)
- [ ] **Step 2:** Baseline testing (5 min)
- [ ] **Phase 1:** Fix debug logging (30 min) ⚡ Low Risk
- [ ] **Phase 2:** Fix font sizing (1-2 hours) ⚠️ Medium Risk
- [ ] **Phase 3:** Fix checkbox marking (2-3 hours) 🔴 High Risk
- [ ] **Phase 4:** Integration testing (1 hour)
- [ ] **Phase 5:** Documentation & merge (30 min)

---

## Step 0: Commit Specification (5 minutes)

```bash
cd /home/mlindegarde/projects/gen-ai--dnd-mcp
git add specs/006-fix-windows-pdf-filling/
git commit -m "spec: Add specification for Windows PDF fixing (006)

- Complete specification with 9 documents (1,861 lines)
- Covers checkbox visibility, font sizing, and debug logging
- Includes implementation guide with code examples
- Ready for development"

git push origin main
```

**Verify:**
```bash
git log --oneline -1  # Should show the spec commit
```

---

## Step 1: Create Feature Branch (2 minutes)

```bash
git checkout -b 006-fix-windows-pdf-filling
git push -u origin 006-fix-windows-pdf-filling
```

**Verify:**
```bash
git branch --show-current
# Output should be: 006-fix-windows-pdf-filling
```

---

## Step 2: Baseline Testing (5 minutes)

**Purpose:** Document current broken state

```bash
# Run tests
cargo test 2>&1 | tee baseline-tests.log

# Generate PDF with current (broken) code
cargo run test

# Save baseline
cp filled_character_sheet.pdf baseline-before-fix.pdf

# Document issues
ls -la mcp_debug.log  # Probably doesn't exist
# Open baseline-before-fix.pdf
# Note: Checkboxes not visible, fonts too large
```

**Expected Results:**
- ❌ `mcp_debug.log` missing
- ❌ Checkboxes not visible in PDF
- ❌ Fonts too large in PDF

---

## Phase 1: Fix Debug Logging (30 minutes)

**Risk Level:** ⚡ LOW

**File:** `src/mcp_server.rs` (lines 11-24)

### Steps:

1. **Backup:**
   ```bash
   cp src/mcp_server.rs src/mcp_server.rs.backup
   ```

2. **Read Implementation Guide:**
   ```bash
   cat specs/006-fix-windows-pdf-filling/IMPLEMENTATION_GUIDE.md | less
   # Navigate to "Phase 1: Debug Logging Fix"
   ```

3. **Edit `src/mcp_server.rs`:**
   
   Replace the `log_to_file` function (lines 11-24) with:
   
   ```rust
   fn log_to_file(message: &str) {
       use std::env;
       use std::path::PathBuf;
       
       // Try project root first, fall back to temp directory
       let log_path: PathBuf = env::current_dir()
           .map(|d| d.join("mcp_debug.log"))
           .unwrap_or_else(|_| env::temp_dir().join("dnd-mcp-debug.log"));
       
       if let Ok(mut file) = OpenOptions::new()
           .create(true)
           .append(true)
           .open(&log_path)
       {
           let _ = writeln!(
               file,
               "[{}] {}",
               chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
               message
           );
       }
   }
   ```

4. **Test:**
   ```bash
   cargo build
   cargo run test
   ls -la mcp_debug.log        # Should exist!
   tail -20 mcp_debug.log      # Should show entries
   ```

5. **Commit:**
   ```bash
   git add src/mcp_server.rs
   git commit -m "fix: Use cross-platform path for debug log

- Replace hardcoded macOS path with env::current_dir()
- Add fallback to temp directory if project root not writable
- Fixes debug log missing on Windows

Issue: Debug log file missing on non-developer systems
Solution: Dynamic path resolution with graceful fallback"
   
   git push origin 006-fix-windows-pdf-filling
   ```

### Success Criteria:
- ✅ `cargo build` succeeds
- ✅ `mcp_debug.log` exists in project root
- ✅ Log contains timestamp entries
- ✅ No test failures introduced

---

## Phase 2: Fix Font Sizing (1-2 hours)

**Risk Level:** ⚠️ MEDIUM

**File:** `src/pdf_filler.rs`

### Steps:

1. **Backup:**
   ```bash
   cp src/pdf_filler.rs src/pdf_filler.rs.backup
   ```

2. **Add Helper Methods** (after line 650 in `impl PdfFiller` block):
   
   ```rust
   /// Determine appropriate font size based on field name
   fn get_font_size_for_field(&self, field_name: &str) -> u8 {
       // Character name and player name - larger for readability
       if field_name.contains("CharacterName") || field_name.contains("PlayerName") {
           return 12;
       }
       
       // Spell fields - smaller to fit more text
       if field_name.starts_with("Spells") {
           return 8;
       }
       
       // Narrative fields - slightly smaller
       if field_name.contains("Traits") || 
          field_name.contains("Features") || 
          field_name.contains("Ideals") ||
          field_name.contains("Bonds") ||
          field_name.contains("Flaws") {
           return 9;
       }
       
       // Default for all other fields
       10
   }

   /// Create PDF default appearance string with font and size
   fn create_default_appearance(&self, field_name: &str) -> String {
       let font_size = self.get_font_size_for_field(field_name);
       format!("/Helv {} Tf 0 g", font_size)
   }
   ```

3. **Update `fill_pdf_fields` method** (around line 129):
   
   Replace:
   ```rust
   if let Some(value) = field_values.get(field_name.as_ref()) {
       let mut new_dict = dict.clone();
       new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
       doc.objects.insert(object_id, Object::Dictionary(new_dict));
   }
   ```
   
   With:
   ```rust
   if let Some(value) = field_values.get(field_name.as_ref()) {
       let mut new_dict = dict.clone();
       
       // Set the field value
       new_dict.set(b"V", Object::String(value.as_bytes().to_vec(), lopdf::StringFormat::Literal));
       
       // Set default appearance with appropriate font size
       let da_string = self.create_default_appearance(&field_name);
       new_dict.set(b"DA", Object::String(da_string.as_bytes().to_vec(), lopdf::StringFormat::Literal));
       
       doc.objects.insert(object_id, Object::Dictionary(new_dict));
   }
   ```

4. **Test:**
   ```bash
   cargo build
   cargo test
   cargo run test
   
   # Save snapshot
   cp filled_character_sheet.pdf after-phase2-fonts.pdf
   
   # Visual inspection of after-phase2-fonts.pdf:
   # - Character name readable (12pt)
   # - Spell names fit (8pt)
   # - No text cutoff
   ```

5. **Commit:**
   ```bash
   git add src/pdf_filler.rs
   git commit -m "fix: Add font size specification to text fields

- Add get_font_size_for_field() to determine appropriate sizes
- Add create_default_appearance() to generate /DA strings
- Set /DA property when filling text fields
- Use 12pt for names, 10pt standard, 8pt for spells

Issue: Font sizes too large in PDF fields
Solution: Set /DA (Default Appearance) with appropriate font sizes"
   
   git push origin 006-fix-windows-pdf-filling
   ```

### Success Criteria:
- ✅ `cargo build` succeeds
- ✅ `cargo test` passes
- ✅ Character name at 12pt
- ✅ Spell fields at 8pt
- ✅ No text cutoff in critical fields

---

## Phase 3: Fix Checkbox Marking (2-3 hours)

**Risk Level:** 🔴 HIGH - Test thoroughly!

**File:** `src/pdf_filler.rs` (mark_checkboxes method, line 169)

### Steps:

1. **Optional - Research checkbox structure:**
   ```bash
   cargo run --bin find_all_checkboxes
   ```

2. **Update `mark_checkboxes` method** (line 169):
   
   Replace:
   ```rust
   fn mark_checkboxes(
       &self,
       doc: &mut Document,
       checkbox_fields: &HashMap<String, bool>,
   ) -> Result<(), PdfError> {
       for (object_id, object) in doc.objects.clone() {
           if let Object::Dictionary(dict) = object {
               if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                   let field_name = String::from_utf8_lossy(&field_name_bytes);
                   
                   if let Some(&should_mark) = checkbox_fields.get(field_name.as_ref()) {
                       if should_mark {
                           let mut new_dict = dict.clone();
                           new_dict.set(b"V", Object::String(b"Yes".to_vec(), lopdf::StringFormat::Literal));
                           doc.objects.insert(object_id, Object::Dictionary(new_dict));
                       }
                   }
               }
           }
       }
       Ok(())
   }
   ```
   
   With:
   ```rust
   fn mark_checkboxes(
       &self,
       doc: &mut Document,
       checkbox_fields: &HashMap<String, bool>,
   ) -> Result<(), PdfError> {
       for (object_id, object) in doc.objects.clone() {
           if let Object::Dictionary(dict) = object {
               if let Ok(Object::String(field_name_bytes, _)) = dict.get(b"T") {
                   let field_name = String::from_utf8_lossy(&field_name_bytes);
                   
                   if let Some(&should_mark) = checkbox_fields.get(field_name.as_ref()) {
                       if should_mark {
                           let mut new_dict = dict.clone();
                           
                           // Use Name objects instead of String for checkbox values
                           // This is required by PDF spec for button fields
                           new_dict.set(b"V", Object::Name(b"Yes".to_vec()));
                           
                           // Set appearance state to match the value
                           // This tells PDF viewers which appearance to display
                           new_dict.set(b"AS", Object::Name(b"Yes".to_vec()));
                           
                           doc.objects.insert(object_id, Object::Dictionary(new_dict));
                       }
                   }
               }
           }
       }
       Ok(())
   }
   ```

3. **Test:**
   ```bash
   cargo build
   cargo test
   cargo run test
   
   # Save snapshot
   cp filled_character_sheet.pdf after-phase3-checkboxes.pdf
   ```

4. **CRITICAL - Visual Verification on Windows:**
   
   Open `after-phase3-checkboxes.pdf` and check:
   - [ ] Saving throw proficiency boxes VISIBLE
   - [ ] Skill proficiency boxes VISIBLE
   - [ ] Prepared spell boxes VISIBLE
   
   Test in multiple viewers:
   - [ ] Adobe Reader (Windows) - **MOST IMPORTANT**
   - [ ] Chrome PDF viewer
   - [ ] Edge PDF viewer

5. **Test on macOS/Linux (regression check):**
   ```bash
   # If you have access to another platform
   cargo test
   cargo run test
   # Verify checkboxes still work
   ```

6. **Commit:**
   ```bash
   git add src/pdf_filler.rs
   git commit -m "fix: Use Name objects and appearance state for checkboxes

- Change checkbox /V from String to Name object
- Add /AS (Appearance State) property matching /V
- Add explanatory comments about PDF spec requirement

Issue: Checkboxes not visible on Windows
Solution: Use Name objects per PDF 1.7 spec for button fields"
   
   git push origin 006-fix-windows-pdf-filling
   ```

### Success Criteria:
- ✅ `cargo build` succeeds
- ✅ `cargo test` passes
- ✅ **All proficiency checkboxes VISIBLE on Windows**
- ✅ Works in Adobe Reader (Windows)
- ✅ Works in Chrome PDF viewer
- ✅ Works in Edge PDF viewer
- ✅ No regression on macOS/Linux

---

## Phase 4: Integration Testing (1 hour)

### Full Test Suite:

```bash
# Run all tests
cargo test --verbose 2>&1 | tee final-tests.log

# Check code quality
cargo clippy -- -D warnings

# Format check
cargo fmt --check
# If needed: cargo fmt

# Generate final test PDF
cargo run test
```

### Complete Verification Checklist:

**Logging:**
- [ ] `mcp_debug.log` exists in project root
- [ ] Log entries have timestamps
- [ ] Log shows server activity

**Font Sizing:**
- [ ] Character name: 12pt (large, readable)
- [ ] Ability scores: 10pt (standard)
- [ ] Spell names: 8pt (compact)
- [ ] Personality traits: 9pt (medium)
- [ ] No text cutoff anywhere

**Checkboxes (CRITICAL):**
- [ ] Saving throw proficiency boxes visible
- [ ] Skill proficiency boxes visible
- [ ] Spell preparation boxes visible
- [ ] Tested in Adobe Reader
- [ ] Tested in Chrome
- [ ] Tested in Edge

**Quality:**
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] No regressions

### Compare All Versions:

You should have these PDFs for comparison:
- `baseline-before-fix.pdf` - Original (broken)
- `after-phase2-fonts.pdf` - Fonts fixed
- `after-phase3-checkboxes.pdf` - All fixed
- `filled_character_sheet.pdf` - Final version

---

## Phase 5: Documentation & Merge (30 minutes)

### Steps:

1. **Update README.md** (optional):
   ```markdown
   ## Windows Compatibility
   
   The D&D character sheet filler is fully compatible with Windows 10/11:
   - ✅ Checkboxes visible in all PDF viewers
   - ✅ Appropriate font sizing
   - ✅ Debug log: `./mcp_debug.log`
   ```

2. **Clean up backup files:**
   ```bash
   rm src/mcp_server.rs.backup
   rm src/pdf_filler.rs.backup
   rm baseline-tests.log
   rm final-tests.log
   # Keep PDF snapshots for documentation
   ```

3. **Final commit** (if documentation updated):
   ```bash
   git add README.md
   git commit -m "docs: Update README with Windows compatibility notes"
   git push origin 006-fix-windows-pdf-filling
   ```

4. **Create Pull Request:**
   - Title: "Fix Windows PDF filling issues (checkboxes, fonts, logging)"
   - Description: 
     ```
     Fixes three critical issues preventing proper PDF generation on Windows:
     
     1. ✅ Checkboxes now visible (Name objects + /AS property)
     2. ✅ Font sizes appropriate (8-12pt with /DA property)
     3. ✅ Debug log works (cross-platform path resolution)
     
     Specification: specs/006-fix-windows-pdf-filling/
     
     Tested on: Windows 10, macOS, Linux
     PDF viewers: Adobe Reader, Chrome, Edge
     ```

5. **Self-review:**
   - [ ] All commits have descriptive messages
   - [ ] All tests pass
   - [ ] Code follows Rust conventions
   - [ ] No debug prints left in code
   - [ ] Documentation updated
   - [ ] Ready for review

6. **Merge to main** (after review):
   ```bash
   git checkout main
   git pull origin main
   git merge 006-fix-windows-pdf-filling
   git push origin main
   ```

7. **Clean up:**
   ```bash
   git branch -d 006-fix-windows-pdf-filling
   git push origin --delete 006-fix-windows-pdf-filling
   ```

---

## Rollback Plan

### Phase-specific rollback:

```bash
# Undo last commit (keep changes)
git reset HEAD~1

# Undo last commit (discard changes)
git reset --hard HEAD~1

# Restore from backup
cp src/mcp_server.rs.backup src/mcp_server.rs
cp src/pdf_filler.rs.backup src/pdf_filler.rs
```

### Nuclear option (complete rollback):

```bash
git checkout main
git branch -D 006-fix-windows-pdf-filling
# Start over from Step 1
```

---

## Troubleshooting

### Log file not created:
```bash
pwd                                    # Verify you're in project root
ls -la .                              # Check write permissions
ls -la /tmp/dnd-mcp-debug.log         # Check fallback location
```

### Checkboxes not visible:
```bash
# Use Adobe Reader (most reliable)
cargo run --bin find_all_checkboxes  # Verify fields exist
# Verify: Used Object::Name not Object::String?
```

### Font issues:
```bash
# Add debug print in create_default_appearance:
println!("Setting DA for {}: {}", field_name, da_string);
```

### Build errors:
```bash
cargo clean
cargo build
# Check import statements
# Check syntax in new code
```

---

## Time Tracking Template

```
Step 0 (Commit spec):    Started: _____ Ended: _____ Duration: _____
Step 1 (Create branch):  Started: _____ Ended: _____ Duration: _____
Step 2 (Baseline):       Started: _____ Ended: _____ Duration: _____
Phase 1 (Logging):       Started: _____ Ended: _____ Duration: _____
Phase 2 (Fonts):         Started: _____ Ended: _____ Duration: _____
Phase 3 (Checkboxes):    Started: _____ Ended: _____ Duration: _____
Phase 4 (Integration):   Started: _____ Ended: _____ Duration: _____
Phase 5 (Documentation): Started: _____ Ended: _____ Duration: _____

Total Duration: _____
```

---

## Ready to Start!

✨ **Begin with Step 0: Commit the specification**

```bash
git add specs/006-fix-windows-pdf-filling/
git commit -m "spec: Add specification for Windows PDF fixing (006)"
git push origin main
```

Then proceed through each phase sequentially, testing after each change.

**Key Reminders:**
- Commit after each phase
- Keep backup files until merge
- Test checkboxes in Adobe Reader on Windows
- Focus on Phase 3 - that's the critical fix

Good luck! 🚀
