# 🔧 自动更新故障排查

## 常见问题及解决方案

### ❌ 错误：updater.check not allowed

**错误信息**：
```
updater.check not allowed. Permissions associated with this command: updater:allow-check, updater:default
```

**原因**：缺少必要的权限配置

**解决方案**：

在 `src-tauri/capabilities/default.json` 中添加以下权限：

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

### ❌ 更新检查失败：404 Not Found

**原因**：GitHub Release 中没有 `latest.json` 文件

**解决方案**：

1. 确保 `build.yml` 中设置了 `includeUpdaterJson: true`
2. 重新发布一个版本，让 Tauri Action 自动生成 `latest.json`
3. 或者手动创建并上传 `latest.json` 到 Release

### ❌ 开发模式下无法检查更新

**原因**：更新功能仅在生产构建中有效

**解决方案**：

这是正常的！使用 `pnpm tauri build` 构建生产版本进行测试。

### ❌ Logo 没有显示红点

**可能原因**：
1. 没有新版本可用
2. 检查更新失败（查看控制台错误）
3. 更新状态未正确传递

**排查步骤**：

1. 打开开发者工具（F12）查看控制台
2. 检查是否有错误信息
3. 验证 `updateStore.hasUpdate` 的值：
   ```javascript
   // 在控制台输入
   console.log(updateStore.hasUpdate)
   ```

### ❌ 下载进度不显示

**原因**：下载进度回调未正确设置

**检查**：`AboutDialog.vue` 中的下载代码：

```javascript
await update.downloadAndInstall((event) => {
    switch (event.event) {
        case 'Started':
            updateStore.setDownloadProgress(0)
            break
        case 'Progress':
            const progress = Math.round((event.data.downloaded / event.data.contentLength) * 100)
            updateStore.setDownloadProgress(progress)
            break
        case 'Finished':
            updateStore.setDownloadProgress(100)
            break
    }
})
```

### ❌ 安装后应用没有重启

**原因**：`relaunch()` 调用失败或缺少权限

**解决方案**：

1. 确保有 `process:allow-restart` 权限
2. 检查是否正确导入：
   ```javascript
   import { relaunch } from '@tauri-apps/plugin-process'
   ```

### ❌ 版本号比较错误

**原因**：版本号格式不一致

**解决方案**：

确保所有地方的版本号格式一致：
- `Cargo.toml`: `version = "1.5.0"`
- `tauri.conf.json`: `"version": "1.5.0"`
- Git 标签: `1.5.0`（不要用 `v1.5.0`）

### ⚠️ Windows Defender 拦截更新

**原因**：未签名的安装包被 Windows Defender 识别为可疑

**解决方案**：

1. 添加代码签名（推荐）
2. 或使用 Tauri 更新包签名：
   ```bash
   pnpm tauri signer generate
   ```
   然后在 `tauri.conf.json` 中添加公钥

## 调试技巧

### 1. 查看更新信息

在控制台运行：

```javascript
const { check } = await import('@tauri-apps/plugin-updater')
const update = await check()
console.log('更新信息:', update)
```

### 2. 手动测试更新流程

```javascript
// 1. 检查更新
const { check } = await import('@tauri-apps/plugin-updater')
const update = await check()

// 2. 查看版本信息
if (update) {
    console.log('当前版本:', update.currentVersion)
    console.log('最新版本:', update.version)
    console.log('更新说明:', update.body)
}

// 3. 下载并安装
if (update) {
    await update.downloadAndInstall((event) => {
        console.log('下载事件:', event)
    })
}

// 4. 重启应用
const { relaunch } = await import('@tauri-apps/plugin-process')
await relaunch()
```

### 3. 验证 latest.json

访问更新端点 URL：
```
https://github.com/caolib/my-tools/releases/latest/download/latest.json
```

应该返回类似以下内容：
```json
{
  "version": "1.5.0",
  "notes": "更新说明",
  "pub_date": "2025-01-02T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "",
      "url": "https://github.com/caolib/my-tools/releases/download/1.5.0/my-tools_1.5.0_x64-setup.nsis.zip"
    }
  }
}
```

## 日志收集

如果问题持续，收集以下信息：

1. **控制台日志**：F12 → Console
2. **网络请求**：F12 → Network
3. **Tauri 日志**：应用数据目录中的日志文件
4. **系统信息**：Windows 版本、应用版本

## 需要帮助？

如果以上方法都无法解决问题：

1. 查看 [Tauri 更新器文档](https://v2.tauri.app/plugin/updater/)
2. 在 [GitHub Issues](https://github.com/caolib/my-tools/issues) 提问
3. 提供详细的错误日志和复现步骤
