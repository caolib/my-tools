# Tauri 环境变量配置说明

## 本地开发

已创建 `.env.local` 文件存储签名私钥,该文件已被 Git 忽略。

### 文件位置
```
.env.local
```

### 环境变量说明

| 变量名 | 说明 | 是否必需 |
|--------|------|----------|
| `TAURI_SIGNING_PRIVATE_KEY` | Tauri 签名私钥(Base64编码) | 打包时必需 |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | 私钥密码 | 如果私钥加密则必需 |

### 使用方式

**Windows PowerShell:**
```powershell
# 方式1: 使用 dotenv
pnpm add -D dotenv
# 在打包脚本中加载: node -r dotenv/config script.js

# 方式2: 手动设置环境变量
$env:TAURI_SIGNING_PRIVATE_KEY = (Get-Content .env.local | Select-String "TAURI_SIGNING_PRIVATE_KEY").ToString().Split("=")[1]
pnpm tauri build
```

**Linux/macOS:**
```bash
# 加载环境变量
export $(cat .env.local | xargs)
pnpm tauri build
```

## GitHub Actions

在 GitHub Repository Settings 中配置 Secrets:

1. 进入仓库设置: `Settings` → `Secrets and variables` → `Actions`
2. 点击 `New repository secret`
3. 添加以下 Secrets:
   - `TAURI_SIGNING_PRIVATE_KEY`: 私钥内容(从 `.env.local` 复制)
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: 私钥密码(如果有)

## 安全注意事项

⚠️ **重要**:
- `.env.local` 包含敏感信息,已添加到 `.gitignore`
- 永远不要提交私钥到 Git 仓库
- 不要在公开场合分享私钥内容
- 定期更换私钥以提高安全性

## 密钥文件位置

原始密钥文件位置:
- 私钥: `C:\Users\caolib\.tauri\my-tools.key`
- 公钥: `C:\Users\caolib\.tauri\my-tools.key.pub`
- 公钥已配置在 `src-tauri/tauri.conf.json` 中

## 构建命令

```bash
# 本地开发构建(需要 .env.local)
pnpm tauri build

# 生成更新文件
pnpm gen  # 等同于 tauri build
```

## 故障排查

### 错误: "No private key found"

**原因**: 未设置 `TAURI_SIGNING_PRIVATE_KEY` 环境变量

**解决方案**:
1. 确认 `.env.local` 文件存在
2. 在 PowerShell 中手动加载环境变量:
   ```powershell
   $envContent = Get-Content .env.local | Where-Object { $_ -match "TAURI_SIGNING_PRIVATE_KEY=" }
   $keyValue = $envContent -replace "TAURI_SIGNING_PRIVATE_KEY=", ""
   $env:TAURI_SIGNING_PRIVATE_KEY = $keyValue
   pnpm gen
   ```

### 错误: "Invalid private key"

**原因**: 私钥格式错误或损坏

**解决方案**:
1. 检查 `.env.local` 中的私钥是否完整
2. 确认私钥没有换行或多余空格
3. 重新从原始文件复制私钥内容
