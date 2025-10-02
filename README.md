# NamsoGen - 现代化信用卡测试号码生成器

🦀 使用 **Rust + Tauri + React** 构建的高性能桌面应用

## ⚠️ 重要声明

**本工具生成的信用卡号码仅用于软件开发、测试和验证目的。这些号码基于 Luhn 算法生成，具有正确的格式，但不对应任何真实的银行账户或资金。严禁将此工具用于任何非法用途或实际交易。**

## ✨ 特性

- 🚀 **极速性能**: Rust 后端，批量生成 10,000 个卡号仅需毫秒级
- 🎨 **现代界面**: React + Tailwind CSS 打造优雅用户体验
- 📦 **跨平台**: 支持 Windows、macOS、Linux
- 💾 **小体积**: 打包后仅 5-10MB，无需运行时环境
- 🔒 **安全可靠**: 纯本地运行，无需网络连接
- ⚡ **功能完整**: 支持所有主流信用卡网络和导出格式

## 🛠️ 技术栈

### 后端 (Rust)
- **Tauri 2.0** - 现代桌面应用框架
- **Serde** - 高效序列化/反序列化
- **Rand** - 随机数生成

### 前端 (TypeScript)
- **React 18** - 现代化 UI 框架
- **Tailwind CSS** - 实用优先的 CSS 框架
- **Zustand** - 轻量级状态管理
- **Lucide React** - 精美图标库

## 🚀 快速开始

### 环境要求
- **Node.js** >= 18.0.0
- **Rust** >= 1.70.0

### 安装运行
```bash
# 克隆项目
git clone <项目地址>
cd namso-gen

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建应用
npm run tauri build
```

## 📋 功能概览

### 支持的卡类型
- ✅ Visa (4开头)
- ✅ Mastercard (51-55, 2221-2720)
- ✅ American Express (34, 37)
- ✅ Discover (6011, 644-649, 65)
- ✅ UnionPay (62)
- ✅ Diners Club (300-305, 36, 38)

### 导出格式
- ✅ **CARD** - 卡片显示格式
- ✅ **PIPE** - 管道分隔格式 (`卡号|过期|CVV`)
- ✅ **CSV** - 表格格式
- ✅ **JSON** - 结构化数据
- ✅ **XML** - 标记语言格式
- ✅ **SQL** - 数据库插入语句

### 核心功能
- 🔢 **Luhn 算法** - 标准信用卡校验算法
- 🎯 **BIN 支持** - 支持自定义银行识别码
- 📅 **自定义过期日期** - 精确控制月份和年份
- 🔐 **CVV 生成** - 根据卡类型生成正确长度
- 💰 **虚拟余额** - 支持 16 种国际货币
- 📊 **批量生成** - 支持 1-10,000 个卡号

### 辅助工具
- ✅ **卡号验证器** - 实时验证卡号有效性
- ✅ **用户数据生成** - 生成测试用户信息
- ✅ **Lorem Ipsum** - 生成占位文本

## 📁 项目结构

```
namso-gen/
├── src/                          # React 前端
│   ├── components/               # UI 组件
│   │   ├── BasicMode.tsx        # 基础模式
│   │   ├── AdvancedMode.tsx     # 高级模式
│   │   ├── ToolsMode.tsx        # 工具模式
│   │   └── ResultPanel.tsx      # 结果面板
│   ├── lib/
│   │   ├── types.ts             # TypeScript 类型
│   │   ├── store.ts             # 状态管理
│   │   └── utils.ts             # 工具函数
│   └── App.tsx                  # 主应用
├── src-tauri/                   # Rust 后端
│   ├── src/
│   │   ├── main.rs              # 程序入口
│   │   ├── card_generator.rs   # 核心生成器
│   │   ├── validator.rs         # 卡号验证
│   │   ├── exporters.rs         # 多格式导出
│   │   └── commands.rs          # Tauri 命令
│   └── Cargo.toml               # Rust 依赖
├── package.json                 # 项目配置
└── README.md                    # 项目文档
```

## 🎯 使用方法

### 基础模式
1. 选择信用卡网络类型
2. 选择导出格式
3. 设置生成参数
4. 点击生成按钮

### 高级模式
- 支持自定义 BIN 码（如 `559888039xxxxxxx`）
- 可设置虚拟余额和货币类型
- 支持更大批量生成

### 工具模式
- **验证器**: 输入卡号验证有效性
- **用户生成**: 生成测试用户数据
- **Lorem Ipsum**: 生成占位文本

## 🔧 开发指南

### 添加新卡类型
1. 在 `src-tauri/src/card_generator.rs` 中添加网络定义
2. 更新前端类型定义
3. 更新 UI 选择器

### 添加新导出格式
1. 在 `src-tauri/src/exporters.rs` 中实现导出函数
2. 在 `commands.rs` 中注册命令
3. 在前端调用新命令

## 📦 构建分发

### GitHub Actions 自动构建

项目配置了 GitHub Actions 来自动构建多平台版本：

- ✅ macOS (Apple Silicon)
- ✅ macOS (Intel)
- ✅ Windows (x64)

#### 触发自动构建

**方法1: 创建发布标签**
```bash
git tag v2.0.1
git push origin v2.0.1
```

**方法2: 手动触发**
1. 进入 GitHub 仓库的 Actions 页面
2. 选择 "Build and Release" 工作流
3. 点击 "Run workflow"

构建完成后，在 GitHub Releases 页面下载对应平台的安装包。

### 本地构建

```bash
# 构建当前平台
npm run tauri build

# Windows 交叉编译（需要额外工具）
npm run tauri build -- --target x86_64-pc-windows-msvc
```

构建产物位置：
- **Windows**: `src-tauri/target/release/bundle/nsis/NamsoGen_*.msi`
- **macOS**: `src-tauri/target/release/bundle/dmg/NamsoGen_*.dmg`
- **Linux**: `src-tauri/target/release/bundle/appimage/NamsoGen_*.AppImage`

## 🐛 故障排除

### 常见问题
1. **Rust 编译错误**: 运行 `rustup update`
2. **依赖安装失败**: 清除 `node_modules` 重新安装
3. **端口占用**: 检查 1420 端口是否可用

详细解决方案请参考项目文档。

## 📄 许可证

本项目仅供学习和测试目的使用。请遵守当地法律法规，合理使用。

## 🙏 致谢

- 灵感来源于 [namso-gen.com](https://namso-gen.com)
- 感谢 Tauri 团队提供的优秀框架
- 感谢 Rust 和 React 社区

---

**⚠️ 再次提醒**: 请仅将此工具用于合法的开发和测试目的！