# 🔥 重要!立即配置 GitHub Secrets

## 问题原因

之前发布的 v1.4.1 **没有生成 `latest.json` 文件**,因为:

1. ❌ `tauri.conf.json` 缺少 `createUpdaterArtifacts: true`
2. ❌ 没有配置签名密钥
3. ❌ GitHub Actions 没有设置签名环境变量

## 已修复 ✅

- ✅ 添加了 `bundle.createUpdaterArtifacts: true`
- ✅ 生成了签名密钥对
- ✅ 公钥已配置到 `tauri.conf.json`
- ✅ 更新了 GitHub Actions 工作流

## 🚨 立即行动!

### 第一步: 配置 GitHub Secrets

1. 打开浏览器访问:
   ```
   https://github.com/caolib/my-tools/settings/secrets/actions
   ```

2. 点击 **"New repository secret"**

3. ⚠️ **重要!配置私钥时请注意以下事项:**

#### 配置 TAURI_SIGNING_PRIVATE_KEY

- **Name**: `TAURI_SIGNING_PRIVATE_KEY`
- **Secret**: 复制以下内容(确保是**一整行**,不要有换行):
  ```
  dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5ZmNZWjRIZTA2c2IyK1JPMkdya2Q2V0xkaHJhT2xYM2lxSS9LejFjS29nWUFBQkFBQUFBQUFBQUFBQUlBQUFBQWd6c25vbnRBYmIvcmZVQ3NXeFpWNGo5QXJHbjlPQ0l6WjZCdlRmcHdyVWtDMndvR3Q0eWhQbktvcWlra0dTdEVaNVlUZUdsdUxQQXc3RUloeTlvWTluUFVhbnVtYm9WN243NmhhUGpDTjVLVVRNdmxGdEVGTUxSalVWbU0yVUljdlNpN3A0cHREazA9Cg==
  ```

**⚠️ 关键步骤:**
1. 全选上面的密钥文本(应该是 348 个字符)
2. Ctrl+C 复制
3. 在 GitHub Secret 输入框中 Ctrl+V 粘贴
4. **检查是否只有一行,没有换行符!**
5. 点击 "Add secret"

#### 配置 TAURI_SIGNING_PRIVATE_KEY_PASSWORD

- **Name**: `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- **Secret**: `你刚才生成密钥时设置的密码`
- 点击 "Add secret"

**如果你忘记了密码,需要重新生成密钥!**

### 第二步: 发布新版本测试

1. 提交当前更改:
   ```powershell
   git add .
   git commit -m "feat: 配置自动更新签名"
   git push
   ```

2. 修改版本号为 1.4.2:
   ```powershell
   # 修改 src-tauri/tauri.conf.json 中的 version
   # 修改 src-tauri/Cargo.toml 中的 version
   ```

3. 打标签并推送:
   ```powershell
   git tag v1.4.2
   git push origin v1.4.2
   ```

4. 观察 GitHub Actions 构建过程

5. 构建完成后,检查 Release 页面应该有这些文件:
   - ✅ `my-tools_1.4.2_x64-setup.exe`
   - ✅ `my-tools_1.4.2_x64-setup.nsis.zip`
   - ✅ `my-tools_1.4.2_x64-setup.nsis.zip.sig`
   - ✅ **`latest.json`** ⭐ 这是关键文件!

### 第三步: 测试更新功能

1. 安装 v1.4.1 版本
2. 运行应用,等待 3 秒
3. 应该看到 logo 上出现红色脉动圆点
4. 点击 logo,在关于对话框点击"更新"
5. 观察下载进度
6. 安装完成后应用自动重启

## 📝 重要提醒

### ⚠️ 密钥安全

- **私钥文件**: `C:\Users\caolib\.tauri\my-tools.key`
  - ❌ 永远不要提交到 Git
  - ❌ 永远不要分享给任何人
  - ✅ 做好备份(断网的 U 盘或加密存储)
  - ❌ 丢失私钥 = 无法发布更新!

- **公钥**: 已经在代码中,可以公开

### 📋 版本发布清单

每次发布新版本前:

- [ ] 更新 `src-tauri/tauri.conf.json` 的 `version`
- [ ] 更新 `src-tauri/Cargo.toml` 的 `version`
- [ ] 更新 `docs/RELEASE.md` 的发布说明
- [ ] 提交代码
- [ ] 打 tag (例如: `v1.4.2`)
- [ ] 推送 tag 到 GitHub
- [ ] 等待 Actions 构建完成
- [ ] 检查 Release 中是否有 `latest.json`
- [ ] 使用旧版本测试更新功能

## 🎯 下一步

1. **现在就去配置 GitHub Secrets** (5分钟)
2. 发布 v1.4.2 测试更新功能
3. 如果成功,v1.4.0 的用户就能自动更新了!

## 📚 详细文档

查看 `docs/UPDATER_SETUP.md` 了解更多细节。
