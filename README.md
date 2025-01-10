# Project Structure Creator

Create a project files and directories structure based on the ASCII tree structure.

## Installation

```bash
cargo install project-structure-creator
```

## Usage

```bash
project-structure-creator --input <path-to-ascii-tree-file>
```

or

```bash
project-structure-creator
# paste the ASCII tree structure here
```

## Example

```bash
project-structure-creator --input ./example.txt
```

### Example ASCII tree structure

```
project
├── src
│   ├── main.rs
│   ├── lib.rs
├── tests
│   ├── test.rs
├── Cargo.toml
├── README.md
```