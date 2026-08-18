Commands
===

1. Init:
  - What init does: Creates a new vault based on the arguments that had been implemented.
  - Which arguments does it takes: 2 obrigatory fields name and path it can be absolute or relative.
  - What it makes: Does a .vault dir and a .vaultbypass file (the file is possible to be added). The .vault has all tracks of the contents and much more, the rest will be detailed below.
  - Structure for the generated contents:
    - .vault:
    ```
    .vault/
    │
    ├── REFER
    ├── config.toml
    ├── index.db
    │
    ├── branches/
    │   ├── trunk/
    │   │   ├── HEAD
    │   │   ├── saves/
    │   │   └── refs/
    │   │
    │   └── example/
    │       ├── HEAD
    │       ├── saves/
    │       └── refs/
    │
    ├── objects/
    │   ├── blobs/
    │   ├── trees/
    │   ├── manifests/
    │   └── metadata/
    │
    ├── info/
    │   ├── vault.json
    │   ├── statistics.json
    │   └── sessions/
    │
    └── temp/
        └── unaudited_saves/
    ```
    - .vaultbypass: utilize ```file_name``` for files, ```dir/``` for folders, for general file types utilize ```*.txt``` for them to be excluded from the vault tracking system, .vault/ is already defined to be bypassed on the system itself.

  2. Open:
  - What open does: Open a different vault at a command
  - Which arguments does it takes: 1 obrigatory argument, the vault name.
  - Dependency: a external file that is currently named store.toml for more information refer to commands_templates.

  3. Add:
  - What does: Adds untracked files into the snapshot system, and prepare them and their metadata to be saved.
  - Which Arguments does it takes: obrigatory boolean of all files and a optional files destinations.
  - Detailed walk through:
    │
  
    │
