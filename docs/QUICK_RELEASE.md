# 🚀 快速发布指南

## 发布新版本的步骤

### 1️⃣ 更新版本号

编辑两个文件中的版本号：

**`src-tauri/Cargo.toml`**
```toml
[package]
version = "1.5.0"  # 修改这里
```

**`src-tauri/tauri.conf.json`**
```json
{
  "version": "1.5.0"  # 修改这里
}
```

### 2️⃣ 编写发布说明

编辑 **`docs/RELEASE.md`**：

```markdown
## 本次更新

### 新功能 ✨
- 添加了自动更新功能
- 实现了一键更新和进度显示

### 改进 🎨
- 优化了启动速度
- 改进了用户界面

### 修复 🐛
- 修复了某个已知问题

---
📋 [查看完整更新日志](https://github.com/caolib/my-tools/compare/v1.4.0...v1.5.0)
```

### 3️⃣ 提交并发布

```bash
# 1. 提交更改
git add .
git commit -m "chore: bump version to 1.5.0"

# 2. 创建标签（注意：不需要 v 前缀）
git tag 1.5.0

# 3. 推送到 GitHub
git push origin main
git push origin 1.5.0
```

### 4️⃣ 等待构建完成

- GitHub Actions 会自动开始构建
- 访问 https://github.com/caolib/my-tools/actions 查看进度
- 构建完成后会自动创建 Release

### 5️⃣ 验证发布

1. 访问 https://github.com/caolib/my-tools/releases
2. 确认新版本已发布
3. 确认包含以下文件：
   - `my-tools_1.5.0_x64-setup.nsis.zip` (安装包)
   - `latest.json` (更新配置文件)

## 📋 检查清单

发布前请确认：

- [ ] 版本号已在 `Cargo.toml` 和 `tauri.conf.json` 中更新
- [ ] 发布说明已写入 `docs/RELEASE.md`
- [ ] 代码已提交到 main 分支
- [ ] 标签格式正确（例如 `1.5.0`，不是 `v1.5.0`）
- [ ] 所有测试通过
- [ ] 本地构建成功

## 🎯 常见问题

### Q: 标签应该用 `v1.5.0` 还是 `1.5.0`？
A: 使用 `1.5.0`（不带 v 前缀），因为 `tauri.conf.json` 中的版本号不带 v。

### Q: 如果构建失败怎么办？
A: 
1. 检查 GitHub Actions 日志
2. 修复问题后重新提交
3. 删除旧标签：`git tag -d 1.5.0 && git push origin :refs/tags/1.5.0`
4. 重新创建标签

### Q: 如何测试更新功能？
A: 
1. 安装旧版本（例如 1.4.0）
2. 发布新版本（例如 1.5.0）
3. 打开应用，等待 3 秒
4. 应该看到 logo 上的红点
5. 点击 logo，应该显示更新信息

### Q: 用户如何更新？
A: 
- **自动提示**：应用启动时会自动检查，有更新时 logo 显示红点
- **手动检查**：点击 logo → 关于对话框 → 检查更新按钮
- **一键更新**：点击"立即更新"按钮，自动下载安装

## 🔗 相关文档

- [自动更新功能说明](./AUTO_UPDATE.md)
- [完整实现文档](./UPDATE_IMPLEMENTATION.md)
- [GitHub Actions 配置](../.github/workflows/build.yml)
