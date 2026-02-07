# OmniShell

OmniShell 是一个为开发者设计的轻量级桌面应用运行时支持库。它的核心目标是解决现代桌面应用由于内置完整浏览器内核而导致的内存占用过高问题。

## 主要功能

- **内核共享**：多个 HTML 应用可以共用一个底层内核进程，极大节省系统资源。
- **全平台互通**：支持跨窗口、跨应用的消息广播与数据同步。
- **原生视觉**：内置了对 Windows Mica（云母）、macOS Vibrancy 等高级毛玻璃特效的支持。
- **全系统兼容**：支持 Windows、macOS 以及各种主流 Linux 发行版。

## 快速开始

### 1. 前提条件

确保您的开发环境已安装以下工具：
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- 对应的系统依赖（参考 [Tauri 文档](https://tauri.app/v1/guides/getting-started/prerequisites)）

### 2. 安装与运行

```bash
# 安装依赖
npm install

# 启动开发模式
npm run tauri dev

# 构建安装包
npm run tauri build
```

## 开发者指南

在您的 HTML 应用中引入支持库：

```javascript
import { Omni } from './lib/omni-bridge.js';

// 打开一个新窗口
Omni.window.open({
    id: 'my-app',
    title: '我的应用',
    url: 'https://example.com'
});

// 向其他窗口发送消息
Omni.bus.emit('message-channel', { text: '你好，世界！' });
```

## 许可证

MIT
