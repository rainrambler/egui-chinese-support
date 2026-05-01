# egui-chinese-support

[![Crates.io](https://img.shields.io/crates/v/egui-chinese-font.svg)](https://crates.io/crates/egui-chinese-font)
[![Documentation](https://docs.rs/egui-chinese-font/badge.svg)](https://docs.rs/egui-chinese-font)
[![License](https://img.shields.io/crates/l/egui-chinese-font.svg)](https://github.com/username/egui-chinese-font#license)

A cross-platform Rust crate for automatically loading Chinese fonts in [egui](https://github.com/emilk/egui) applications.

## Features

- 🌍 **Cross-platform**: Works on Windows, macOS, and Linux
- 🔤 **Automatic font detection**: Automatically finds and loads system Chinese fonts
- 🎨 **Easy integration**: Simple one-line setup with egui
- 📝 **Multiple formats**: Supports both Simplified and Traditional Chinese
- ⚡ **Lightweight**: Minimal dependencies and fast loading
- 🛠️ **Flexible**: Supports custom font loading

## Supported Platforms

| Platform | Fonts Detected |
|----------|----------------|
| Windows  | Microsoft YaHei, SimSun, SimHei, KaiTi, FangSong, Microsoft JhengHei |
| macOS    | PingFang SC, STHeiti, Hiragino Sans GB, Arial Unicode MS |
| Linux    | Noto Sans CJK, WQY MicroHei, Droid Sans Fallback, AR PL UMing |

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
egui-chinese-support = "0.1"
egui = "0.34"
```

### Basic Usage

```rust
use egui_chinese_font::setup_chinese_fonts;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chinese Font Example",
        options,
        Box::new(|cc| {
            // Setup Chinese fonts - this is all you need!
            setup_chinese_fonts(&cc.egui_ctx).expect("Failed to load Chinese fonts");
            
            Box::new(MyApp::default())
        }),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好世界！"); // This will now display correctly
            ui.label("这是中文测试文本。");
            ui.label("Traditional Chinese: 繁體中文測試");
        });
    }
}
```

### Custom Font Loading

If you want to use your own Chinese font file:

```rust
use egui_chinese_font::setup_custom_chinese_font;

// Load your custom font
let font_data = std::fs::read("path/to/your/chinese_font.ttf").unwrap();
setup_custom_chinese_font(&ctx, font_data, Some("my_chinese_font"));
```

### Error Handling

```rust
use egui_chinese_font::{setup_chinese_fonts, FontError};

match setup_chinese_fonts(&ctx) {
    Ok(()) => println!("Chinese fonts loaded successfully"),
    Err(FontError::NotFound(msg)) => eprintln!("No Chinese fonts found: {}", msg),
    Err(FontError::ReadError(err)) => eprintln!("Failed to read font file: {}", err),
    Err(FontError::UnsupportedPlatform) => eprintln!("Platform not supported"),
}
```

## API Reference

### Functions

- `setup_chinese_fonts(ctx: &egui::Context) -> Result<(), FontError>` - Automatically detect and load system Chinese fonts
- `setup_custom_chinese_font(ctx: &egui::Context, font_data: Vec<u8>, font_name: Option<&str>)` - Load custom Chinese font data
- `get_chinese_font_paths() -> Vec<String>` - Get list of potential Chinese font paths for debugging

### Error Types

- `FontError::NotFound(String)` - No suitable Chinese fonts found on the system
- `FontError::ReadError(std::io::Error)` - Error reading font file
- `FontError::UnsupportedPlatform` - Current platform is not supported

## Examples

See the [`examples/`](examples/) directory for complete working examples:

- [`basic.rs`](examples/basic.rs) - Simple Chinese text display
- Run with: `cargo run --example basic`

## Platform-Specific Notes

### Windows
- Looks for Microsoft YaHei (recommended), SimSun, SimHei, and other system fonts
- Fonts are typically located in `C:\Windows\Fonts\`

### macOS
- Prefers PingFang SC and STHeiti fonts
- Falls back to Hiragino Sans GB and Arial Unicode MS

### Linux
- Searches for Noto Sans CJK, WQY fonts, and Droid Sans Fallback
- Font availability varies by distribution
- Install Chinese fonts: `sudo apt install fonts-noto-cjk` (Ubuntu/Debian)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

**Contact**: 259901434@qq.com

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Changelog

### 0.1.0
- Initial release
- Cross-platform Chinese font loading
- Support for Windows, macOS, and Linux
- Automatic system font detection
- Custom font loading support

## 中文说明 / Chinese Documentation

### 概述

`egui-chinese-support` 是一个专为 [egui](https://github.com/emilk/egui) GUI 框架设计的中文字体加载库。它能够自动检测并加载系统中的中文字体，让你的 Rust GUI 应用程序完美显示中文文本。

### 主要特性

- 🌐 **跨平台支持**: 支持 Windows、macOS 和 Linux 系统
- 🇨🇳 **智能字体检测**: 自动识别和加载系统中可用的中文字体
- 🎨 **简单集成**: 只需一行代码即可完成中文字体配置
- 📝 **全面支持**: 支持简体中文、繁体中文和混合文本
- ⚡ **轻量高效**: 最小化依赖，快速加载，不影响应用性能
- 🛠️ **灵活配置**: 支持自定义字体文件和字体优先级设置
- �️ **安全可靠**: 完整的错误处理机制，类型安全的 API 设计

### 支持的字体系统

| 操作系统 | 支持的字体 | 备注 |
|---------|-----------|------|
| **Windows** | 微软雅黑、宋体、黑体、楷体、仿宋、微软正黑体 | 推荐使用微软雅黑 |
| **macOS** | 苹方简体、华文黑体、冬青黑体简体中文、Arial Unicode MS | 系统内置字体 |
| **Linux** | Noto Sans CJK、文泉驿微米黑、文泉驿正黑、文鼎PL明体 | 需要安装中文字体包 |

### 快速开始

1. **添加依赖**

在你的 `Cargo.toml` 文件中添加：

```toml
[dependencies]
egui-chinese-support = "0.1"
egui = "0.34"
eframe = "0.34"  # 如果你使用 eframe
```

2. **基础使用**

```rust
use egui_chinese_font::setup_chinese_fonts;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "我的中文应用",
        options,
        Box::new(|cc| {
            // 设置中文字体 - 这是关键的一行！
            setup_chinese_fonts(&cc.egui_ctx)
                .expect("无法加载中文字体");
            
            Box::new(MyApp::default())
        }),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好，世界！");
            ui.label("这是中文文本显示测试");
            ui.label("支持简体中文：北京欢迎您");
            ui.label("支持繁體中文：台北歡迎您");
            ui.label("支持混合文本：Hello 世界 2025");
        });
    }
}
```

### 高级用法

#### 自定义字体加载

```rust
use egui_chinese_font::setup_custom_chinese_font;

// 从文件加载自定义字体
let font_data = std::fs::read("assets/my_chinese_font.ttf")?;
setup_custom_chinese_font(&ctx, font_data, Some("我的字体"));

// 或者从内嵌资源加载
let font_data = include_bytes!("../assets/chinese_font.ttf").to_vec();
setup_custom_chinese_font(&ctx, font_data, Some("内嵌字体"));
```

#### 错误处理最佳实践

```rust
use egui_chinese_font::{setup_chinese_fonts, FontError};

match setup_chinese_fonts(&ctx) {
    Ok(()) => {
        println!("✅ 中文字体加载成功！");
    },
    Err(FontError::NotFound(msg)) => {
        eprintln!("❌ 未找到中文字体: {}", msg);
        // 可以尝试加载备用字体或显示警告
    },
    Err(FontError::ReadError(e)) => {
        eprintln!("❌ 读取字体文件失败: {}", e);
    },
    Err(FontError::UnsupportedPlatform) => {
        eprintln!("❌ 当前平台不支持自动字体检测");
    }
}
```

#### 调试字体问题

```rust
use egui_chinese_font::get_chinese_font_paths;

// 检查系统中可用的中文字体路径
let font_paths = get_chinese_font_paths();
println!("可用的中文字体路径:");
for (i, path) in font_paths.iter().enumerate() {
    println!("  {}. {}", i + 1, path);
    // 检查文件是否存在
    if std::path::Path::new(path).exists() {
        println!("     ✅ 存在");
    } else {
        println!("     ❌ 不存在");
    }
}
```

### 常见问题解决

#### 1. 中文显示为方块或乱码
- **原因**: 系统缺少中文字体或字体加载失败
- **解决**: 
  - 检查 `setup_chinese_fonts()` 是否成功调用
  - 使用 `get_chinese_font_paths()` 检查可用字体
  - 考虑使用自定义字体文件

#### 2. Linux 系统无法显示中文
- **原因**: Linux 发行版可能未预装中文字体
- **解决**:
  ```bash
  # Ubuntu/Debian
  sudo apt install fonts-noto-cjk fonts-wqy-microhei
  
  # CentOS/RHEL
  sudo yum install google-noto-cjk-fonts wqy-microhei-fonts
  
  # Arch Linux
  sudo pacman -S noto-fonts-cjk wqy-microhei
  ```

#### 3. 字体显示效果不理想
- **原因**: 系统默认字体可能不是最佳选择
- **解决**: 使用 `setup_custom_chinese_font()` 加载高质量字体文件

### 性能优化建议

1. **字体加载时机**: 在应用启动时一次性加载，避免运行时重复加载
2. **内存使用**: 大字体文件会占用较多内存，根据需要选择合适的字体
3. **平台适配**: 为不同平台准备不同的字体文件，提供最佳用户体验

### 许可证说明

本项目采用双重许可证：
- **MIT 许可证**: 适用于商业和开源项目
- **Apache 2.0 许可证**: 提供专利保护

你可以根据项目需要选择其中一种许可证。

### 贡献指南

欢迎为这个项目做出贡献！你可以：

1. 🐛 **报告问题**: 在 GitHub Issues 中报告 bug
2. 💡 **提出建议**: 分享你的改进想法
3. 🔧 **提交代码**: 通过 Pull Request 贡献代码
4. � **完善文档**: 帮助改进文档和示例
5. 🌐 **添加字体支持**: 为更多平台和字体提供支持

**联系方式**: 259901434@qq.com

### 更新日志

#### 版本 0.1.0 (2025-06-25)
- ✨ 首次发布
- ✅ 支持 Windows、macOS、Linux 三大平台
- ✅ 自动检测和加载系统中文字体
- ✅ 支持自定义字体文件加载
- ✅ 完整的错误处理和类型安全 API
- ✅ 提供调试和故障排除工具
- 📚 完整的中英文文档和示例代码

---

## Features (English)

- 🌐 Cross-platform support (Windows, macOS, Linux)
- 🇨🇳 Automatic detection and loading of system Chinese fonts
- 🎨 Support for custom font data
- 📝 Simple and easy-to-use API
- 🛡️ Type-safe error handling

## Installation / 安装

Add this to your `Cargo.toml`:

```toml
[dependencies]
egui-chinese-support = "0.1.0"
egui = "0.27.0"
```

## Usage / 使用方法

### 基本用法 / Basic Usage

```rust
use egui_chinese_font::setup_chinese_fonts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "我的中文应用",
        options,
        Box::new(|cc| {
            // 设置中文字体
            if let Err(e) = setup_chinese_fonts(&cc.egui_ctx) {
                eprintln!("加载中文字体失败: {}", e);
            }
            Box::new(MyApp::default())
        }),
    )?;
    Ok(())
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好，世界！");
            ui.label("这是中文文本显示测试");
        });
    }
}
```

### 使用自定义字体 / Using Custom Fonts

```rust
use egui_chinese_font::setup_custom_chinese_font;

// 加载自定义字体文件
let font_data = std::fs::read("path/to/your/chinese_font.ttf")?;
setup_custom_chinese_font(&ctx, font_data, Some("my_chinese_font"));
```

### 检查可用字体路径 / Check Available Font Paths

```rust
use egui_chinese_font::get_chinese_font_paths;

let font_paths = get_chinese_font_paths();
for path in font_paths {
    println!("可能的字体路径: {}", path);
}
```

## 支持的字体 / Supported Fonts

---

*这个库为 egui Rust GUI 框架提供了完整的中文支持，让开发者能够轻松创建支持中文显示的现代化应用程序。*

*This library provides complete Chinese language support for the egui Rust GUI framework, enabling developers to easily create modern applications with Chinese text display.*
