# Files

## Virtual File System (VFS)

The primary way to access files within your node is through the [VFS API](./apis/vfs.md).
The VFS API follows std::fs closely, adding some capabilities checks on paths and some combinatory actions.

VFS files exist in the "/vfs" folder within your home node, and files are grouped by `package_id`.
For example, part of the VFS might look like:

```text
.
├── app_store:nectar
│   └── pkg
│       ├── app_store.wasm
│       ├── ft_worker.wasm
│       ├── manifest.json
│       └── metadata.json
├── chess:nectar
│   └── pkg
│       ├── chess.html
│       ├── chess.wasm
│       ├── index.css
│       ├── index.js
│       ├── manifest.json
│       └── metadata.json
├── homepage:nectar
│   └── pkg
│       ├── homepage.wasm
│       ├── manifest.json
│       └── metadata.json
```

## Usage

To access files in the vfs, you need to create or open a drive, this can be done with the function `create_drive` from the standard library:

```rust
let drive_path: String = create_drive(our.package_id(), "drive_name")?;
```

All examples are using the [nectar_process_lib](./process_stdlib/overview.md) functions, and would be imported like
```rust
use nectar_process_lib::vfs::{open_file, create, ...};
```

### Files

#### Open a File

```rust
/// Opens a file at path, if no file at path, creates one if boolean create is true.
let file_path = format!("{}/hello.txt", &drive_path);
let file = open_file(&file_path, true);
```

#### Create a File

```rust
/// Creates a file at path, if file found at path, truncates it to 0.
let file_path = format!("{}/hello.txt", &drive_path);
let file = create(&file_path);
```

#### Read a File

```rust
/// Reads the entire file, from start position.
/// Returns a vector of bytes.
let contents = file.read()?;
```

#### Write a File

```rust
/// Write entire slice as the new file.
/// Truncates anything that existed at path before.
let buffer = b"Hello!";
file.write(&buffer)?;
```

#### Write to File

```rust
/// Write buffer to file at current position, overwriting any existing data.
let buffer = b"World!";
file.write_at(&buffer)?;
```

#### Read at position

```rust
/// Read into buffer from current cursor position
/// Returns the amount of bytes read.
let mut buffer = vec![0; 5];
file.read_at(&buffer)?;
```

#### Set Length

```rust
/// Set file length, if given size > underlying file, fills it with 0s.
file.set_len(42)?;
```

#### Seek to a position

```rust
/// Seek file to position.
/// Returns the new position.
let position = SeekFrom::End(0);
file.seek(&position)?;
```

#### Sync

```rust
/// Syncs path file buffers to disk.
file.sync_all()?;
```

#### Metadata

```rust
/// Metadata of a path, returns file type and length.
let metadata = file.metadata()?;
```

### Directories

#### Open a Directory

```rust
/// Opens or creates a directory at path.
/// If trying to create an existing file, will just give you the path.
let dir_path = format!("{}/my_pics", &drive_path);
let dir = open_dir(&file_path, true);
```

#### Read a Directory

```rust
/// Iterates through children of directory, returning a vector of DirEntries.
/// DirEntries contain the path and file type of each child.
let entries = dir.read()?;
```

#### General path Metadata

```rust
/// Metadata of a path, returns file type and length.
let some_path = format!("{}/test", &drive_path);
let metadata = metadata(&some_path)?;
```

## References

- [VFS API](./apis/vfs.md)
- [std::fs API](https://doc.rust-lang.org/std/fs/index.html)
