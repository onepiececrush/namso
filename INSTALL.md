# 安装说明

## 📦 预编译版本（推荐）

### Windows

1. **下载安装包**
   - `NamsoGen-2.0.0-x64-setup.msi` (64位推荐)
   - 或 `NamsoGen-2.0.0-x86-setup.msi` (32位)

2. **运行安装程序**
   - 双击 `.msi` 文件
   - 按提示完成安装
   - 可能需要管理员权限

3. **启动应用**
   - 开始菜单 → NamsoGen
   - 或桌面快捷方式

**系统要求**：
- Windows 10/11
- WebView2 Runtime（通常已预装）

### macOS

1. **下载安装包**
   - `NamsoGen-2.0.0-universal.dmg` (推荐，支持 Intel 和 M1/M2)
   - 或 `NamsoGen-2.0.0-x64.dmg` (仅 Intel)
   - 或 `NamsoGen-2.0.0-aarch64.dmg` (仅 Apple Silicon)

2. **安装应用**
   - 双击 `.dmg` 文件
   - 将 NamsoGen.app 拖到 Applications 文件夹
   - 首次打开时可能需要：
     - 右键点击 → 打开
     - 或在"系统偏好设置 → 安全性"中允许

3. **启动应用**
   - 从 Applications 文件夹启动
   - 或使用 Spotlight 搜索 "NamsoGen"

**系统要求**：
- macOS 10.13 或更高版本

### Linux

#### 方法一：AppImage（推荐）

1. **下载 AppImage**
   ```bash
   wget https://github.com/.../NamsoGen-2.0.0-x86_64.AppImage
   ```

2. **添加执行权限**
   ```bash
   chmod +x NamsoGen-2.0.0-x86_64.AppImage
   ```

3. **运行应用**
   ```bash
   ./NamsoGen-2.0.0-x86_64.AppImage
   ```

#### 方法二：DEB 包（Debian/Ubuntu）

```bash
# 下载并安装
wget https://github.com/.../namso-gen_2.0.0_amd64.deb
sudo dpkg -i namso-gen_2.0.0_amd64.deb

# 如果缺少依赖
sudo apt-get install -f

# 启动
namso-gen
```

#### 方法三：从源码编译

```bash
# 安装依赖
sudo apt install -y libwebkit2gtk-4.0-dev \
    build-essential curl wget libssl-dev \
    libgtk-3-dev libayatana-appindicator3-dev \
    librsvg2-dev

# 克隆并构建
git clone <repository-url>
cd tauri-rewrite
npm install
npm run tauri build

# 安装包位于 src-tauri/target/release/bundle/
```

**系统要求**：
- Ubuntu 20.04+ / Debian 11+
- 或其他支持 GTK3 的发行版

---

## 🔧 从源码安装

### 前置要求

1. **安装 Node.js**
   ```bash
   # 推荐使用 nvm
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   ```

2. **安装 Rust**
   ```bash
   # Unix (macOS/Linux)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Windows
   # 下载并运行: https://win.rustup.rs/
   ```

3. **验证安装**
   ```bash
   node --version   # 应该 >= 18.0.0
   npm --version
   rustc --version  # 应该 >= 1.70.0
   cargo --version
   ```

### 克隆项目

```bash
git clone <repository-url>
cd tauri-rewrite
```

### 安装依赖

```bash
npm install
```

### 运行开发版本

```bash
npm run tauri dev
```

### 构建生产版本

```bash
npm run tauri build
```

构建产物位于：
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/deb/` 或 `appimage/`

---

## 🔍 安装验证

安装完成后，验证应用是否正常工作：

### 1. 启动测试
- 应用能够正常启动
- 窗口正常显示
- 无错误提示

### 2. 功能测试
```
✅ 生成卡号（基础模式）
✅ 生成卡号（高级模式）
✅ 验证卡号
✅ 复制到剪贴板
✅ 保存文件
```

### 3. 性能测试
```bash
# 测试大批量生成
1. 切换到高级模式
2. 数量选择 10000
3. 点击生成
4. 应该在 1 秒内完成
```

---

## 🐛 故障排除

### Windows

**问题 1**: 提示缺少 WebView2
```
解决: 下载并安装 WebView2 Runtime
https://developer.microsoft.com/microsoft-edge/webview2/
```

**问题 2**: 安装时提示需要管理员权限
```
解决: 右键 .msi 文件 → 以管理员身份运行
```

**问题 3**: Windows Defender 误报
```
解决: 
1. 打开 Windows 安全中心
2. 病毒和威胁防护 → 保护历史记录
3. 允许该应用
```

### macOS

**问题 1**: "无法打开，因为无法验证开发者"
```
解决:
1. 系统偏好设置 → 安全性与隐私
2. 点击"仍要打开"
或
1. 右键点击应用
2. 选择"打开"
3. 确认打开
```

**问题 2**: 应用已损坏
```
解决: 移除隔离属性
xattr -cr /Applications/NamsoGen.app
```

**问题 3**: 权限问题
```
解决: 授予必要权限
chmod +x /Applications/NamsoGen.app/Contents/MacOS/NamsoGen
```

### Linux

**问题 1**: 缺少依赖
```
解决: Ubuntu/Debian
sudo apt-get install -f
sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0
```

**问题 2**: AppImage 无法运行
```
解决:
1. 检查执行权限: chmod +x NamsoGen.AppImage
2. 安装 FUSE: sudo apt install fuse libfuse2
```

**问题 3**: 字体显示问题
```
解决: 安装字体
sudo apt install fonts-noto fonts-noto-cjk
```

### 源码安装问题

**问题 1**: Rust 编译错误
```
解决:
# 更新 Rust
rustup update

# 清理并重新构建
cd src-tauri
cargo clean
cargo build
```

**问题 2**: Node 依赖安装失败
```
解决:
# 清理缓存
rm -rf node_modules package-lock.json
npm cache clean --force
npm install
```

**问题 3**: 构建时内存不足
```
解决:
# 限制并行任务
export CARGO_BUILD_JOBS=2
npm run tauri build
```

---

## 📊 系统要求详细说明

### 最低配置
- **CPU**: 1 GHz 双核
- **RAM**: 2 GB
- **磁盘**: 50 MB 可用空间
- **显示**: 1024x768 分辨率

### 推荐配置
- **CPU**: 2 GHz 四核或更高
- **RAM**: 4 GB 或更多
- **磁盘**: 100 MB 可用空间
- **显示**: 1920x1080 分辨率

### 操作系统
| 平台 | 最低版本 | 推荐版本 |
|------|---------|---------|
| Windows | 10 | 11 |
| macOS | 10.13 | 13+ |
| Linux | Ubuntu 20.04 | 22.04+ |

---

## 🔄 更新应用

### 检查更新
应用会在启动时检查更新（如果启用）。

### 手动更新
1. 下载最新版本安装包
2. 卸载旧版本（可选，覆盖安装会保留设置）
3. 安装新版本

### 保留数据
应用配置保存在：
- **Windows**: `%APPDATA%\com.namsogen.app\`
- **macOS**: `~/Library/Application Support/com.namsogen.app/`
- **Linux**: `~/.config/com.namsogen.app/`

---

## 🗑️ 卸载应用

### Windows
```
控制面板 → 程序和功能 → 选择 NamsoGen → 卸载
```

### macOS
```
将 /Applications/NamsoGen.app 移到废纸篓
```

### Linux
```bash
# DEB 包
sudo apt remove namso-gen

# AppImage
rm NamsoGen.AppImage
```

### 清理配置文件（可选）
```bash
# Windows
rmdir /s %APPDATA%\com.namsogen.app

# macOS
rm -rf ~/Library/Application\ Support/com.namsogen.app

# Linux
rm -rf ~/.config/com.namsogen.app
```

---

## 📞 获取帮助

安装遇到问题？

1. **查看文档**
   - [快速开始](QUICKSTART.md)
   - [使用指南](使用指南.md)
   - [故障排除](#-故障排除)

2. **提交 Issue**
   - 访问 GitHub Issues
   - 描述问题和系统信息
   - 附上错误日志（如有）

3. **社区支持**
   - 查看常见问题
   - 搜索已有问题
   - 参与讨论

---

**祝安装顺利！** 🎉


