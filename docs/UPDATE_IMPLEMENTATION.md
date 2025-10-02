# 自动更新功能实现总结

## ✅ 已完成的配置

### 1. 后端配置（Rust）

- ✅ 在 `Cargo.toml` 中添加了依赖：
  - `tauri-plugin-updater = "2.0.0"` (仅桌面平台)
  - `tauri-plugin-notification = "2.0.0"`

- ✅ 在 `lib.rs` 中注册了插件：
  ```rust
  .plugin(tauri_plugin_notification::init())
  .plugin(tauri_plugin_updater::Builder::new().build())
  ```

### 2. 前端配置（JavaScript）

- ✅ 安装了 npm 包：
  - `@tauri-apps/plugin-updater`
  - `@tauri-apps/plugin-notification`

- ✅ 创建了更新状态管理：
  - `src/stores/update.js` - Pinia store 管理更新状态

- ✅ 实现了自动检查功能：
  - `App.vue` - 启动时延迟 3 秒自动检查更新

- ✅ 实现了 UI 提示：
  - `Titlebar.vue` - Logo 右上角显示红色提示点（带动画）
  - `AboutDialog.vue` - 显示更新信息、进度和更新按钮

### 3. Tauri 配置

- ✅ 在 `tauri.conf.json` 中配置了 updater：
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

### 4. GitHub Actions

- ✅ 修改了 `.github/workflows/build.yml`：
  - 添加了 `includeUpdaterJson: true` 自动生成 `latest.json`
  - 配置了从 `docs/RELEASE.md` 读取发布说明

## 📋 功能特性

1. **启动检查** - 应用启动 3 秒后自动检查更新
2. **红点提示** - 发现新版本时 logo 显示红色动画提示点
3. **版本信息** - 关于对话框显示当前版本和新版本信息
4. **手动检查** - 提供"检查更新"按钮手动触发检查
5. **进度显示** - 下载时显示实时进度条
6. **自动安装** - 下载完成后自动安装并重启应用
7. **静默安装** - Windows 使用 passive 模式静默安装

## 🚀 使用方法

### 发布新版本

1. 更新版本号：
   ```bash
   # 修改 src-tauri/Cargo.toml 和 src-tauri/tauri.conf.json
   # version = "1.5.0"
   ```

2. 编写发布说明：
   ```bash
   # 编辑 docs/RELEASE.md
   ```

3. 提交并创建标签：
   ```bash
   git add .
   git commit -m "chore: bump version to 1.5.0"
   git tag 1.5.0
   git push origin 1.5.0
   ```

4. GitHub Actions 自动：
   - 构建应用
   - 生成 `latest.json`
   - 创建 Release
   - 上传安装包

### 用户体验流程

1. 用户打开应用
2. 应用启动 3 秒后自动检查更新（后台进行）
3. 如果有更新，logo 显示红色提示点
4. 用户点击 logo 打开关于对话框
5. 对话框显示新版本信息和"立即更新"按钮
6. 点击更新按钮，显示下载进度
7. 下载完成后自动安装
8. 安装完成后应用自动重启

## � 权限配置（重要）

在 `src-tauri/capabilities/default.json` 中添加了必要的权限：

```json
{
  "permissions": [
    // ... 其他权限
    "updater:default",
    "updater:allow-check",
    "notification:default",
    "notification:allow-is-permission-granted",
    "notification:allow-request-permission",
    "notification:allow-notify",
    "notification:allow-show",
    "process:default",
    "process:allow-restart"
  ]
}
```

**权限说明**：
- `updater:default` - 更新器默认权限
- `updater:allow-check` - 允许检查更新
- `notification:*` - 通知相关权限（可选，用于更新提醒）
- `process:allow-restart` - 允许重启应用（更新完成后）

## �🔧 高级配置（可选）

### 添加更新包签名

为了确保更新包的安全性，可以添加签名验证：

1. 生成密钥对：
   ```bash
   pnpm tauri signer generate
   ```

2. 将公钥添加到 `tauri.conf.json`：
   ```json
   {
     "plugins": {
       "updater": {
         "pubkey": "您的公钥内容"
       }
     }
   }
   ```

3. 将私钥添加到 GitHub Secrets：
   - 设置名称：`TAURI_PRIVATE_KEY`
   - 值：私钥内容

4. 取消注释 `build.yml` 中的环境变量：
   ```yaml
   env:
     TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
   ```

## 📝 注意事项

1. ✅ 更新功能仅在生产构建中有效（`pnpm tauri build`）
2. ✅ 开发模式下不会检查更新（避免干扰开发）
3. ✅ 确保 GitHub Release 是公开的
4. ✅ 首次使用需要先发布一个基础版本
5. ✅ 版本号使用标签格式（如 `1.5.0`，不是 `v1.5.0`）
6. ✅ 更新包必须是 `.nsis.zip` 格式

## 🎉 完成状态

所有自动更新功能已经完整实现并配置好，可以直接使用！

下次发布新版本时，只需：
1. 更新版本号
2. 编写 `docs/RELEASE.md`
3. 创建并推送标签

用户就能自动收到更新提示并一键更新了！
