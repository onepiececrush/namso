# 构建指南

## 快速构建

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

## 详细构建步骤

### 1. 环境准备

#### Windows

```bash
# 安装 Rust
# 从 https://rustup.rs/ 下载安装器

# 安装 Visual Studio Build Tools
# 从 https://visualstudio.microsoft.com/downloads/ 下载

# 安装 WebView2
# 从 https://developer.microsoft.com/microsoft-edge/webview2/ 下载
```

#### macOS

```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Linux (Ubuntu/Debian)

```bash
# 安装系统依赖
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 安装项目依赖

```bash
cd tauri-rewrite
npm install
```

### 3. 开发构建

```bash
npm run tauri dev
```

这会启动：
- Vite 开发服务器在 http://localhost:1420
- Tauri 应用窗口

### 4. 生产构建

```bash
npm run tauri build
```

构建过程：
1. Vite 构建前端（优化、压缩）
2. Cargo 构建 Rust 后端（release 模式）
3. 打包为平台特定的安装程序

### 5. 构建产物

构建完成后，可执行文件和安装包位于：

#### Windows
```
src-tauri/target/release/
├── namso-gen.exe          # 可执行文件
└── bundle/
    ├── msi/              # MSI 安装包
    └── nsis/             # NSIS 安装包
```

#### macOS
```
src-tauri/target/release/
├── bundle/
    ├── macos/
    │   └── NamsoGen.app  # 应用程序
    └── dmg/
        └── NamsoGen.dmg  # DMG 安装镜像
```

#### Linux
```
src-tauri/target/release/
├── namso-gen             # 可执行文件
└── bundle/
    ├── deb/              # Debian 包
    └── appimage/         # AppImage
```

## 平台特定构建

### 构建 Windows 版本（在 Windows 上）

```bash
# 64位版本
npm run tauri build -- --target x86_64-pc-windows-msvc

# 32位版本
rustup target add i686-pc-windows-msvc
npm run tauri build -- --target i686-pc-windows-msvc
```

### 构建 macOS 版本（在 macOS 上）

```bash
# Intel 芯片
npm run tauri build -- --target x86_64-apple-darwin

# Apple Silicon (M1/M2)
npm run tauri build -- --target aarch64-apple-darwin

# 通用版本（同时支持 Intel 和 Apple Silicon）
npm run tauri build -- --target universal-apple-darwin
```

### 构建 Linux 版本（在 Linux 上）

```bash
# 64位版本
npm run tauri build -- --target x86_64-unknown-linux-gnu

# ARM64 版本
rustup target add aarch64-unknown-linux-gnu
npm run tauri build -- --target aarch64-unknown-linux-gnu
```

## 优化构建

### 减小体积

在 `src-tauri/Cargo.toml` 中的 `[profile.release]` 已经配置了优化选项：

```toml
[profile.release]
panic = "abort"        # 禁用栈展开，减小体积
codegen-units = 1      # 更好的优化，构建时间更长
lto = true             # 链接时优化
opt-level = "z"        # 体积优化
strip = true           # 移除调试符号
```

### 更快的构建

开发时使用 `dev` profile（默认）：

```bash
# 开发构建（更快，体积更大）
cargo build

# 发布构建（更慢，体积更小）
cargo build --release
```

### 缓存加速

```bash
# 使用 sccache 加速 Rust 编译
cargo install sccache
export RUSTC_WRAPPER=sccache
```

## 代码签名和公证

### macOS 代码签名

```bash
# 在 tauri.conf.json 中配置
{
  "bundle": {
    "macOS": {
      "signingIdentity": "你的签名身份",
      "entitlements": "entitlements.plist"
    }
  }
}

# 构建并签名
npm run tauri build
```

### Windows 代码签名

```bash
# 在 tauri.conf.json 中配置
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "证书指纹",
      "digestAlgorithm": "sha256"
    }
  }
}
```

## 持续集成 (CI)

### GitHub Actions 示例

创建 `.github/workflows/build.yml`:

```yaml
name: Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [ubuntu-latest, macos-latest, windows-latest]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 18
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install dependencies (Ubuntu)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.0-dev \
            build-essential curl wget libssl-dev \
            libgtk-3-dev libayatana-appindicator3-dev \
            librsvg2-dev
      
      - name: Install Node dependencies
        run: npm install
      
      - name: Build
        run: npm run tauri build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.platform }}
          path: src-tauri/target/release/bundle/
```

## 故障排除

### 构建失败

```bash
# 清理并重新构建
cd src-tauri
cargo clean
cd ..
rm -rf node_modules
npm install
npm run tauri build
```

### 内存不足

```bash
# 减少并行构建任务
export CARGO_BUILD_JOBS=2
npm run tauri build
```

### WebView2 错误 (Windows)

确保安装了 WebView2 Runtime：
https://developer.microsoft.com/microsoft-edge/webview2/

### 权限问题 (macOS)

```bash
# 允许应用程序运行
xattr -cr NamsoGen.app
```

### 依赖冲突

```bash
# 更新 Rust 工具链
rustup update

# 更新 npm 依赖
npm update
```

## 发布检查清单

- [ ] 更新版本号（package.json 和 tauri.conf.json）
- [ ] 更新 CHANGELOG.md
- [ ] 运行所有测试
- [ ] 构建所有平台版本
- [ ] 验证安装包
- [ ] 测试基本功能
- [ ] 代码签名（如需要）
- [ ] 创建 GitHub Release
- [ ] 上传构建产物

## 性能基准

构建时间（参考）：

| 平台 | 开发构建 | 生产构建 |
|------|---------|---------|
| macOS M1 | ~30秒 | ~2分钟 |
| Windows | ~45秒 | ~3分钟 |
| Linux | ~40秒 | ~2.5分钟 |

最终体积（压缩后）：

| 平台 | 体积 |
|------|------|
| Windows (MSI) | ~8-10 MB |
| macOS (DMG) | ~6-8 MB |
| Linux (AppImage) | ~10-12 MB |

## 更多资源

- [Tauri 官方文档](https://tauri.app/v1/guides/)
- [Rust 构建指南](https://doc.rust-lang.org/cargo/guide/)
- [Vite 构建文档](https://vitejs.dev/guide/build.html)


