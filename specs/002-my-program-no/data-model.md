# Data Model: PDF File Saving Fix

**Date**: 2025-10-14  
**Feature**: PDF File Saving Fix  
**Phase**: 1 - Design

## Core Entities

### PDF Output File
**Purpose**: Represents the final filled character sheet PDF file on the filesystem

**Fields**:
- `path`: String - Full filesystem path including filename and extension
- `size_bytes`: u64 - File size in bytes after writing
- `created_at`: SystemTime - Timestamp when file was created
- `is_valid`: bool - Whether post-write validation passed

**Validation Rules**:
- Path must be valid for target operating system
- File extension must be ".pdf"
- Parent directory must exist or be creatable
- Size must be > 0 bytes and < 10MB limit

**State Transitions**:
- Pending → Writing → Validating → Complete
- Any state → Failed (on error)

### Output Path Configuration
**Purpose**: Encapsulates file path validation and directory management

**Fields**:
- `target_path`: PathBuf - Resolved absolute path for output file
- `parent_dir`: PathBuf - Parent directory path
- `filename`: String - Just the filename portion
- `exists`: bool - Whether file already exists at path
- `permissions`: FilePermissions - Read/write access status

**Validation Rules**:
- Path length must be within OS limits
- No invalid characters for target filesystem
- Parent directory must be writable
- Filename must not be empty

### Save Operation Result
**Purpose**: Comprehensive result of PDF save operation with detailed error information

**Fields**:
- `success`: bool - Whether operation completed successfully
- `output_path`: Option<PathBuf> - Path where file was saved (if successful)
- `error_type`: Option<SaveErrorType> - Specific error category if failed
- `error_message`: String - Human-readable error description
- `suggestions`: Vec<String> - Actionable suggestions for resolving errors

**Error Types**:
- `InvalidPath` - Path validation failed
- `PermissionDenied` - Insufficient filesystem permissions
- `DiskSpaceFull` - Insufficient storage space
- `DirectoryCreationFailed` - Could not create parent directories
- `PdfWriteFailed` - PDF library write operation failed
- `IntegrityCheckFailed` - Post-write validation failed

## Relationships

```
Character Data → PDF Filler → Output Path Config → PDF Output File
                     ↓
              Save Operation Result
```

## Data Flow

1. **Input Validation**: Character data and output path are validated
2. **Path Processing**: Output path is resolved and validated
3. **Pre-flight Checks**: Directory existence, permissions, disk space
4. **PDF Generation**: Character data is written to PDF using lopdf
5. **File Writing**: PDF bytes are written to filesystem
6. **Integrity Check**: Saved file is validated for correctness
7. **Result Generation**: Success or detailed error information is returned

## Error Handling Strategy

Each operation stage can fail with specific error types that map to user-actionable messages:

- **Path errors** → Suggest valid path formats and check permissions
- **Permission errors** → Suggest running with appropriate permissions or choosing different location
- **Space errors** → Suggest freeing disk space or choosing different location
- **PDF errors** → Suggest checking character data validity or reporting bug
- **Integrity errors** → Suggest retrying operation or checking system resources
