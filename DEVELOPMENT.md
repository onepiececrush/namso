# 开发指南

## 项目结构说明

```
tauri-rewrite/
├── src/                          # 前端源码 (React + TypeScript)
│   ├── components/               # React 组件
│   │   ├── BasicMode.tsx        # 基础模式组件
│   │   ├── AdvancedMode.tsx     # 高级模式组件
│   │   ├── ToolsMode.tsx        # 工具模式组件
│   │   └── ResultPanel.tsx      # 结果显示面板
│   ├── lib/                     # 工具库
│   │   ├── types.ts             # TypeScript 类型定义
│   │   ├── store.ts             # Zustand 状态管理
│   │   └── utils.ts             # 工具函数和 Tauri 命令调用
│   ├── App.tsx                  # 主应用组件
│   ├── main.tsx                 # React 入口
│   └── index.css                # 全局样式
├── src-tauri/                   # Rust 后端
│   ├── src/
│   │   ├── main.rs              # Tauri 主入口
│   │   ├── card_generator.rs   # 信用卡生成器核心
│   │   ├── validator.rs         # 卡号验证器
│   │   ├── exporters.rs         # 导出功能
│   │   ├── fake_data.rs         # 虚假数据生成
│   │   └── commands.rs          # Tauri 命令定义
│   ├── Cargo.toml               # Rust 依赖配置
│   └── tauri.conf.json          # Tauri 应用配置
├── package.json                 # NPM 依赖配置
├── vite.config.ts               # Vite 构建配置
├── tailwind.config.js           # Tailwind CSS 配置
└── README.md                    # 项目说明文档
```

## 开发环境搭建

### 1. 安装 Rust

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行: https://win.rustup.rs/
```

### 2. 安装 Node.js

推荐使用 Node.js 18+ 版本：

```bash
# 使用 nvm 安装
nvm install 18
nvm use 18
```

### 3. 克隆项目并安装依赖

```bash
cd tauri-rewrite
npm install
```

## 开发流程

### 启动开发服务器

```bash
npm run tauri dev
```

这会同时启动：
- Vite 开发服务器（热重载前端）
- Tauri 应用窗口（连接到开发服务器）

### 前端开发

前端使用 React + TypeScript + Tailwind CSS：

1. **修改组件**: 编辑 `src/components/` 中的文件
2. **更新状态**: 修改 `src/lib/store.ts`
3. **添加类型**: 在 `src/lib/types.ts` 中定义
4. **调用后端**: 使用 `src/lib/utils.ts` 中的函数

热重载会自动生效。

### 后端开发

后端使用 Rust + Tauri：

1. **修改核心逻辑**: 编辑 `src-tauri/src/card_generator.rs`
2. **添加新命令**: 在 `src-tauri/src/commands.rs` 中定义
3. **注册命令**: 在 `src-tauri/src/main.rs` 的 `invoke_handler!` 中添加
4. **前端调用**: 在 `src/lib/utils.ts` 中添加对应的调用函数

修改 Rust 代码后需要重启开发服务器。

## 添加新功能

### 示例：添加新的导出格式

#### 1. 后端实现

在 `src-tauri/src/exporters.rs` 中添加：

```rust
pub fn export_to_yaml(cards: &[CardData]) -> Result<String, String> {
    // 实现 YAML 导出逻辑
    Ok("YAML content".to_string())
}
```

在 `src-tauri/src/commands.rs` 的 `export_cards` 函数中添加：

```rust
"YAML" => Ok(export_to_yaml(&cards)?),
```

#### 2. 前端类型定义

在 `src/lib/types.ts` 中更新：

```typescript
export type ExportFormat = 'CARD' | 'PIPE' | 'CSV' | 'JSON' | 'XML' | 'SQL' | 'YAML';
```

#### 3. 前端 UI 更新

在相应组件中添加 YAML 选项。

## 测试

### 运行 Rust 测试

```bash
cd src-tauri
cargo test
```

### 手动测试清单

- [ ] 基础模式生成
- [ ] 高级模式生成（含 BIN）
- [ ] 所有导出格式
- [ ] 卡号验证
- [ ] 虚假用户数据生成
- [ ] Lorem Ipsum 生成
- [ ] 复制到剪贴板
- [ ] 保存到文件

## 构建发布

### 开发构建

```bash
npm run tauri build
```

### 生产构建

```bash
# 确保 Cargo.toml 中的 release profile 已优化
npm run tauri build -- --release
```

构建产物位于：
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/deb/` 或 `appimage/`

## 调试技巧

### 前端调试

1. 打开浏览器开发者工具（Tauri 窗口中右键 → Inspect）
2. 使用 `console.log()` 输出调试信息
3. 使用 React DevTools 检查组件状态

### 后端调试

1. 使用 `println!()` 或 `dbg!()` 宏输出
2. 查看终端输出
3. 使用 Rust 调试器（VS Code + rust-analyzer）

### 常见问题

**问题**: Rust 编译错误
```bash
# 清理并重新构建
cd src-tauri
cargo clean
cargo build
```

**问题**: 前端无法连接到后端
```bash
# 检查 Tauri 插件是否正确安装
npm run tauri info
```

**问题**: 样式不生效
```bash
# 重新生成 Tailwind CSS
npm run dev
```

## 代码规范

### TypeScript

- 使用 ESLint 检查
- 遵循 Airbnb 风格指南
- 所有函数必须有类型注解

### Rust

- 使用 `cargo fmt` 格式化
- 使用 `cargo clippy` 检查
- 遵循 Rust 官方风格指南

## 性能优化

### 前端优化

1. 使用 React.memo 避免不必要的重渲染
2. 大列表使用虚拟滚动
3. 代码分割和懒加载

### 后端优化

1. 批量生成使用并行处理（rayon）
2. 避免不必要的字符串分配
3. 使用 `cargo build --release` 构建

## 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License


