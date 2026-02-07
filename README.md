# OmniShell Support Library

OmniShell 是一个为现代桌面应用设计的**轻量级运行时支持库**。它不是一个终端应用，而是为 HTML/JS 开发者提供原生能力的 **SDK** 和 **Rust 内核组件**。

## 核心架构

- **`packages/sdk`**: 前端 JavaScript SDK，用于在 HTML 应用中调用原生功能。
- **`src-tauri`**: Rust 核心库，提供跨平台玻璃特效、内核共享和全局消息总线。
- **`examples`**: 使用 OmniShell 库构建的应用示例。

## 如何集成到您的项目

### 1. 引入前端 SDK

您可以将 `packages/sdk/index.js` 复制到您的项目中，或者通过 npm 引用：

```javascript
import { Omni } from './omnishell/sdk/index.js';

// 启动一个受管窗口
Omni.window.open({
    id: 'sub-app',
    title: '子应用',
    url: 'https://your-app-url.com'
});

// 跨应用消息同步
Omni.bus.emit('sync-data', { key: 'value' });
```

### 2. 后端内核集成

如果您正在开发自己的 Tauri 应用并希望使用 OmniShell 的能力，可以将 `src-tauri` 中的核心逻辑作为插件引入。

## 主要特性

| 特性 | 描述 |
| :--- | :--- |
| **内核共享** | 多个窗口共享同一个 Rust 进程，显著降低内存占用 |
| **原生视觉** | 自动适配 Windows Mica (Win11) 和 macOS Vibrancy |
| **全局总线** | 跨窗口、跨应用的事件同步机制 |
| **无边框支持** | 内置自定义拖拽区域支持，实现现代 UI 视觉 |

## 开发与贡献

1. 克隆仓库：`git clone https://github.com/jia6261/OmniShell.git`
2. 查看示例：`cd examples/launcher`
3. 运行示例：需配合外层 Tauri 环境运行。

## 许可证

MIT
