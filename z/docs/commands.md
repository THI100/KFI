Commands
===

1. Init:
 - What init does: Creates a new vault based on the arguments that had been implemented.
 - Which arguments does it takes: 2 obrigatory fields name and path it can be absolute or relative.
 - What it makes: Does a .vault dir and a .vaultbypass file (the file is possible to be added). The .vault has all tracks of the contents and much more, the rest will be detailed below.
 - Structure for the generated contents:
  - .vault:
  ```
  REFER (contains: ./branches/trunk)
  config.toml
  branches/
  trunk/
    saves/
    refs/
  objects/
  info/
  index
  ```
  - .vaultbypass: utilize ```file_name``` for files and ```dir/``` for folders for them to be excluded from the vault tracking system, .vault/ is already defined to be bypassed on the system itself.
