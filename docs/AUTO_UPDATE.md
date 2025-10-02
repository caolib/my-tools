# 自动更新功能说明

## 功能特性

1. **自动检查更新**：应用启动后会自动检查 GitHub Releases 中的最新版本
2. **红点提示**：如果有新版本，logo 右上角会显示红色提示点
3. **手动检查**：在关于对话框中可以手动检查更新
4. **一键更新**：点击"立即更新"按钮可自动下载并安装新版本
5. **进度显示**：更新过程中会显示下载进度
6. **自动重启**：更新完成后自动重启应用

## GitHub Actions 配置

项目已经配置了 `.github/workflows/build.yml`，其中包含了自动更新所需的配置：

```yaml
- uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    # TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }} # 可选：用于更新包签名验证
  with:
    tagName: __VERSION__
    releaseName: 'Release v__VERSION__'
    releaseBody: ${{ steps.release_notes.outputs.body }}
    releaseDraft: false
    prerelease: false
    includeDebug: false
    includeRelease: true
    includeUpdaterJson: true  # 关键：自动生成 latest.json
```

**重要配置说明**：
- `includeUpdaterJson: true` - 自动生成 `latest.json` 文件
- 发布说明从 `docs/RELEASE.md` 文件读取
- 只构建 Windows 版本（`runs-on: windows-latest`）

### 手动创建 latest.json（备用方案）

在每次发布时，需要在 GitHub Releases 中上传 `latest.json` 文件：

```json
{
  "version": "1.4.0",
  "notes": "修复 bug 并添加新功能",
  "pub_date": "2025-01-02T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "",
      "url": "https://github.com/caolib/my-tools/releases/download/v1.4.0/my-tools_1.4.0_x64-setup.nsis.zip"
    }
  }
}
```

## 发布流程

### 自动发布（推荐）

1. 更新版本号：
   - 修改 `src-tauri/Cargo.toml` 中的 `version`
   - 修改 `src-tauri/tauri.conf.json` 中的 `version`

2. 更新发布说明：
   - 编辑 `docs/RELEASE.md` 文件，添加本次更新的内容

3. 提交更改：
   ```bash
   git add .
   git commit -m "chore: bump version to 1.5.0"
   ```

4. 创建并推送标签：
   ```bash
   git tag 1.5.0
   git push origin 1.5.0
   ```

5. GitHub Actions 会自动：
   - 构建 Windows 安装包
   - 生成 `latest.json` 文件
   - 创建 GitHub Release
   - 上传所有文件

### 手动发布（不推荐）

1. 构建应用：`pnpm tauri build`
2. 在 GitHub 创建新的 Release
3. 上传构建产物（`.nsis.zip` 文件）
4. 创建并上传 `latest.json` 文件
5. 发布 Release

## 配置说明

### tauri.conf.json

```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://github.com/caolib/my-tools/releases/latest/download/latest.json"
      ],
      "dialog": false,
      "pubkey": "",
      "windows": {
        "installMode": "passive"
      }
    }
  }
}
```

- `active`: 启用更新功能
- `endpoints`: 更新信息的 JSON 文件地址
- `dialog`: 是否显示 Tauri 内置的更新对话框（我们使用自定义 UI，设为 false）
- `pubkey`: 签名公钥（可选，用于验证更新包）
- `installMode`: Windows 安装模式（passive=静默安装）

### capabilities/default.json（重要）

必须在权限配置中添加更新器权限：

```json
{
  "permissions": [
    "updater:default",
    "updater:allow-check",
    "notification:default",
    "process:default",
    "process:allow-restart"
  ]
}
```

**⚠️ 注意**：如果没有这些权限，更新功能将无法工作，会出现 "updater.check not allowed" 错误。

## 签名（可选但推荐）

为了确保更新包的安全性，可以添加签名：

1. 生成密钥对：
   ```bash
   pnpm tauri signer generate
   ```

2. 将公钥添加到 `tauri.conf.json` 的 `pubkey` 字段

3. 使用私钥对更新包签名（Tauri Action 会自动处理）

4. 将私钥添加到 GitHub Secrets（TAURI_PRIVATE_KEY）

## 测试更新功能

1. 构建并安装当前版本
2. 增加版本号并发布新版本到 GitHub
3. 打开应用，等待自动检查更新
4. 应该看到 logo 上的红点提示
5. 点击 logo 查看更新信息
6. 点击"立即更新"测试更新流程

## 注意事项

1. 更新功能仅在生产构建中有效（`pnpm tauri build`）
2. 开发模式下不会检查更新
3. 确保 GitHub Release 是公开的
4. 更新包必须是 `.nsis.zip` 格式（Windows）
5. 首次使用需要发布一个基础版本作为起点
