# 自动更新配置说明

## 1. 配置已完成 ✅

- ✅ `tauri.conf.json` 已添加公钥和 `createUpdaterArtifacts: true`
- ✅ GitHub Actions 已配置签名环境变量
- ✅ 前端更新检查逻辑已实现

## 2. 需要配置 GitHub Secrets 🔑

前往 https://github.com/caolib/my-tools/settings/secrets/actions 添加以下密钥:

### TAURI_SIGNING_PRIVATE_KEY
```
dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5ZmNZWjRIZTA2c2IyK1JPMkdya2Q2V0xkaHJhT2xYM2lxSS9LejFjS29nWUFBQkFBQUFBQUFBQUFBQUlBQUFBQWd6c25vbnRBYmIvcmZVQ3NXeFpWNGo5QXJHbjlPQ0l6WjZCdlRmcHdyVWtDMndvR3Q0eWhQbktvcWlra0dTdEVaNVlUZUdsdUxQQXc3RUloeTlvWTluUFVhbnVtYm9WN243NmhhUGpDTjVLVVRNdmxGdEVGTUxSalVWbU0yVUljdlNpN3A0cHREazA9Cg==
```

### TAURI_SIGNING_PRIVATE_KEY_PASSWORD
```
你设置的密码
```

## 3. 密钥文件位置 📁

- 私钥: `C:\Users\caolib\.tauri\my-tools.key` ⚠️ **请妥善保管,不要泄露!**
- 公钥: `C:\Users\caolib\.tauri\my-tools.key.pub`

## 4. 发布新版本流程 🚀

1. 修改 `src-tauri/tauri.conf.json` 中的版本号
2. 修改 `src-tauri/Cargo.toml` 中的版本号
3. 更新 `docs/RELEASE.md` 发布说明
4. 提交代码并打 tag:
   ```bash
   git tag v1.4.2
   git push origin v1.4.2
   ```
5. GitHub Actions 会自动构建并生成:
   - `my-tools_1.4.2_x64-setup.exe` - 安装程序
   - `my-tools_1.4.2_x64-setup.nsis.zip` - 更新包
   - `my-tools_1.4.2_x64-setup.nsis.zip.sig` - 签名文件
   - `latest.json` - 更新清单 ✨

## 5. 更新机制说明 📱

- 应用启动 3 秒后自动检查更新
- 如果有更新,logo 显示红色脉动小圆点
- 点击 logo 打开关于对话框
- 在关于对话框中点击"更新"按钮下载安装
- 下载过程显示进度条
- 安装完成后自动重启应用

## 6. 测试更新功能 🧪

1. 确保 GitHub Secrets 已配置
2. 发布一个新版本 (例如 v1.4.2)
3. 使用旧版本 (v1.4.1) 运行应用
4. 等待 3 秒后检查 logo 是否显示红点
5. 打开控制台查看更新检查日志

## 7. 注意事项 ⚠️

- **不要丢失私钥文件和密码!** 否则无法发布更新
- 私钥文件已添加到 `.gitignore`,不会被提交
- 公钥是公开的,可以安全分享
- 所有用户必须使用带签名的版本才能自动更新
- 首次发布带签名的版本后,所有后续版本都必须签名

## 8. 故障排查 🔍

### 更新检查失败
- 检查网络连接
- 确认 `latest.json` 文件存在于 GitHub Release 中
- 查看浏览器控制台错误信息

### 签名验证失败
- 确认公钥正确配置在 `tauri.conf.json`
- 确认 GitHub Secrets 中的私钥正确
- 确认密码正确

### latest.json 未生成
- 确认 `bundle.createUpdaterArtifacts: true` 已配置
- 确认 GitHub Actions 中环境变量已设置
- 检查 Actions 构建日志
