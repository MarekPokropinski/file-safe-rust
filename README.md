Project allows to create safe directory that can be encrypted in fast and simple way.

To create directory run `safe-init` binary:
```bash
$ safe-init <password>
```

This will generate decrypted directory at `$HOME/safe/decrypted safe`. Password will be stored in plain text when directory is decrypted.

To encrypt directory simply run:
```bash
$ safe-encrypt
```
Encrypted directory will be saved to single file: `$HOME/safe/encrypted`.

To decrypt directory run:
```bash
$ safe-decrypt <password>
```
