## 🚀 GitHub Action: SSH/SCP Deployment & Remote Code Execution

GitHub Action for quick deployment using SCP and command execution Pre/Post transfer. Ideal for straightforward CI/CD.

### 💡 Usage

Add this step to your main workflow. Use the latest stable version from the Marketplace (`@v1`) or the main branch (`@main`).

```yaml
- name: Deploy and Remote Code Execution
  uses: alan-venv/ssh-deploy@v1 # Use the latest version tag
  with:
    TARGETS: "dist build/server.js" # Relative paths separated by spaces
    REMOTE_DIR: "/home/user/server"
    REMOTE_KEY: ${{ secrets.SERVER_KEY }}
    REMOTE_HOST: ${{ secrets.SERVER_HOST }}
    REMOTE_USER: ${{ secrets.SERVER_USER }}
    REMOTE_PORT: ${{ secrets.SERVER_PORT }} # Optional: Default 22
    SCRIPT_BEFORE: |
      echo "Starting deployment: Stopping web container..."
      docker compose down
    SCRIPT_AFTER: |
      docker compose up -d --build
      echo "Deployment complete!"
```

### ⚙️ Inputs

| Name | Required | Default | Description |
| :--- | :--- | :--- | :--- |
| `REMOTE_HOST` | Yes | N/A | The hostname or IP address of the target server. |
| `REMOTE_USER` | Yes | N/A | The username for SSH login. |
| `REMOTE_KEY` | Yes | N/A | The SSH private key. **Must be stored as a GitHub Secret.** |
| `TARGETS` | Yes | N/A | The local file or directory path(s) (relative to `$GITHUB_WORKSPACE`) to be copied. Multiple paths should be separated by a space. |
| `REMOTE_DIR` | Yes | N/A | The absolute path of the destination directory on the server. |
| `REMOTE_PORT` | No | `22` | The SSH port of the server. |
| `SCRIPT_BEFORE` | No | N/A | Commands to be executed before the file transfer. |
| `SCRIPT_AFTER` | No | N/A | Commands to be executed after the file transfer. |

### 🔑 SSH Key Setup

The private key must adhere to the following rules for automated execution:

1.  **Format:** The key must be generated in the **PEM** (or modern OpenSSH) format.
    ```bash
    ssh-keygen -m PEM -t rsa -b 4096
    ```
2.  **Passphrase:** **Do not set a passphrase** for the private key, as the project does not support password entry via the command line.
3.  **Security:** The private key (`REMOTE_KEY`) must be stored **exclusively** within [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets).
4.  **Public Key:** The corresponding public key must be added to the `~/.ssh/authorized_keys` file on the target server.

### ⚠️ Important Notes on Commands (Scripts)

  * **Sequential Execution:** Commands within `SCRIPT_BEFORE` and `SCRIPT_AFTER` are executed sequentially on the remote server.
  * **Interruption:** If any command in `SCRIPT_BEFORE` or `SCRIPT_AFTER` returns an exit code **other than zero**, the GitHub Actions step will **fail immediately**, and the workflow will be interrupted.
