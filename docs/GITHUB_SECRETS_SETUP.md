# GitHub Secrets 配置检查清单

## 📋 配置步骤

### 1. 访问 Secrets 页面
打开浏览器,访问:
```
https://github.com/caolib/my-tools/settings/secrets/actions
```

### 2. 检查现有 Secrets
查看页面中是否已存在:
- [ ] `TAURI_SIGNING_PRIVATE_KEY`
- [ ] `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

**如果已存在但构建失败,说明配置有误,需要删除重新添加!**

### 3. 配置 TAURI_SIGNING_PRIVATE_KEY

#### 步骤 A: 获取私钥内容
在本地电脑运行:
```powershell
cat $env:USERPROFILE\.tauri\my-tools.key
```

输出应该是这样(348个字符的单行):
```
dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5ZmNZWjRIZTA2c2IyK1JPMkdya2Q2V0xkaHJhT2xYM2lxSS9LejFjS29nWUFBQkFBQUFBQUFBQUFBQUlBQUFBQWd6c25vbnRBYmIvcmZVQ3NXeFpWNGo5QXJHbjlPQ0l6WjZCdlRmcHdyVWtDMndvR3Q0eWhQbktvcWlra0dTdEVaNVlUZUdsdUxQQXc3RUloeTlvWTluUFVhbnVtYm9WN243NmhhUGpDTjVLVVRNdmxGdEVGTUxSalVWbU0yVUljdlNpN3A0cHREazA9Cg==
```

#### 步骤 B: 添加到 GitHub
1. 点击 **"New repository secret"**
2. **Name**: 输入 `TAURI_SIGNING_PRIVATE_KEY`
   - ⚠️ 必须完全一致,区分大小写!
3. **Secret**: 粘贴私钥内容
   - ⚠️ 确保是单行,没有换行!
   - ⚠️ 前后没有多余空格!
4. 点击 **"Add secret"**

#### 验证方法
添加后,页面应该显示:
```
TAURI_SIGNING_PRIVATE_KEY
Updated X seconds ago
```

### 4. 配置 TAURI_SIGNING_PRIVATE_KEY_PASSWORD

#### 步骤 A: 回忆密码
你在运行 `pnpm tauri signer generate` 时输入的密码。

**如果忘记了密码:**
```powershell
# 重新生成密钥(会覆盖旧的)
pnpm tauri signer generate -w $env:USERPROFILE\.tauri\my-tools.key --force

# 然后需要更新 tauri.conf.json 中的公钥
cat $env:USERPROFILE\.tauri\my-tools.key.pub
```

#### 步骤 B: 添加到 GitHub
1. 点击 **"New repository secret"**
2. **Name**: 输入 `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
   - ⚠️ 必须完全一致,区分大小写!
3. **Secret**: 输入密码
   - ⚠️ 不要有空格!
4. 点击 **"Add secret"**

### 5. 验证配置

#### 本地验证脚本
```powershell
# 运行验证脚本
.\scripts\verify-updater-keys.ps1
```

#### 触发测试构建
```powershell
# 重新触发失败的构建
git tag -d v1.4.2  # 删除本地标签
git push origin :refs/tags/v1.4.2  # 删除远程标签
git tag v1.4.2  # 重新打标签
git push origin v1.4.2  # 推送标签
```

或者手动触发:
1. 访问: https://github.com/caolib/my-tools/actions
2. 选择 "publish" workflow
3. 点击 "Run workflow"
4. 选择 main 分支
5. 点击 "Run workflow" 确认

### 6. 检查构建日志

访问: https://github.com/caolib/my-tools/actions

查看最新的构建日志,应该看到:
```
✓ 签名更新包成功
✓ 生成 latest.json
✓ 上传到 Release
```

### 7. 验证 Release

访问: https://github.com/caolib/my-tools/releases/latest

应该包含文件:
- [ ] `my-tools_1.4.2_x64-setup.exe`
- [ ] `my-tools_1.4.2_x64-setup.nsis.zip`
- [ ] `my-tools_1.4.2_x64-setup.nsis.zip.sig` ⭐
- [ ] `latest.json` ⭐

## 🚨 常见错误

### 错误 1: "Missing comment in secret key"
**原因**: 私钥格式不正确或密码错误

**解决**:
1. 重新复制私钥,确保单行无换行
2. 检查密码是否正确
3. 如果忘记密码,重新生成密钥

### 错误 2: "incorrect updater private key password"
**原因**: 密码不正确

**解决**:
1. 回忆正确的密码
2. 或者重新生成密钥对

### 错误 3: Secret 不存在
**原因**: GitHub Secrets 未配置

**解决**:
按照上面的步骤配置

### 错误 4: 权限问题
**原因**: GitHub Actions 没有权限访问 Secrets

**解决**:
1. 检查仓库设置 → Actions → General
2. 确保 "Workflow permissions" 设置正确

## 📞 获取帮助

如果仍然失败:
1. 检查 GitHub Actions 日志
2. 确认 Secrets 配置正确
3. 尝试重新生成密钥对
4. 查看 Tauri 官方文档: https://v2.tauri.app/plugin/updater/

---

## ✅ 配置成功标志

当你看到:
- ✅ GitHub Secrets 页面显示两个密钥
- ✅ 构建日志没有签名相关错误
- ✅ Release 中包含 `.sig` 和 `latest.json` 文件
- ✅ 应用能检测到更新

恭喜!配置成功! 🎉
