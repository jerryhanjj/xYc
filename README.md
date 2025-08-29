# XML和YANG文件统计工具

一个用Rust编写的命令行工具，用于统计XML和YANG文件的行数、字符数、注释行和空白行。输出格式类似于tokei等专业代码统计工具。

## 功能特性

- 📊 统计XML和YANG文件的行数和字符数信息
- 🎨 美观的表格格式输出（类似tokei工具）
- 🔍 支持文件类型过滤（XML、YANG或全部）
- 📁 支持目录递归搜索
- 📋 详细文件列表显示（英文表头）
- 🌈 彩色终端输出
- 🚀 默认使用当前目录，无需强制指定路径

## 安装

```bash
git clone <repository-url>
cd xYc
cargo build --release
```

## 使用方法

### 基本用法

```bash
# 统计当前目录中的XML和YANG文件（不递归）
./target/release/xYc

# 递归搜索当前目录的所有子目录
./target/release/xYc --recursive

# 显示详细的文件列表并递归搜索
./target/release/xYc --detailed --recursive

# 只统计XML文件并递归搜索
./target/release/xYc --type xml --recursive

# 只统计YANG文件并显示详细信息
./target/release/xYc --type yang --detailed --recursive

# 指定特定目录并递归搜索
./target/release/xYc --path /path/to/directory --recursive
```

### 命令行选项

- `-p, --path <PATH>`: 要统计的目录或文件路径（默认为当前目录）
- `-r, --recursive`: 递归搜索子目录
- `-d, --detailed`: 显示每个文件的详细信息
- `-t, --type <TYPE>`: 指定文件类型 (xml, yang, all) [默认: all]
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

## 输出示例

### 简洁模式
```
统计摘要:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language     Files        Lines        Characters   Comments     Blanks      
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 XML          2            69           2158         0            0           
 YANG         1            158          4606         0            28          
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total        3            227          6764         0            28          
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 详细模式
```
详细文件列表:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 File Path                                    Type    Lines   Characters Comments Blanks  
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 .\test_files\config.xml                      XML     40      1288      0       0       
 .\test_files\data.xml                        XML     29      870       0       0       
 .\test_files\interface.yang                  YANG    158     4606      0       28      
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## 支持的文件类型

- **XML文件** (.xml): 可扩展标记语言文件
- **YANG文件** (.yang): 网络配置数据建模语言文件

## 统计项目说明

- **Language**: 文件语言类型（XML、YANG）
- **Files**: 文件数量
- **Lines**: 总行数
- **Characters**: 总字符数
- **Comments**: 注释行数
- **Blanks**: 空白行数

## 快速开始

```bash
# 克隆项目
git clone <repository-url>
cd xYc

# 构建项目
cargo build --release

# 统计当前目录的文件（不递归）
./target/release/xYc

# 递归搜索所有子目录
./target/release/xYc --recursive

# 显示详细信息并递归搜索
./target/release/xYc --detailed --recursive
```

## 开发

### 依赖项

- `clap`: 命令行参数解析
- `colored`: 彩色终端输出
- `walkdir`: 目录递归遍历

### 构建

```bash
cargo build
```

### 测试

```bash
# 运行单元测试
cargo test

# 测试工具功能
cargo run -- --detailed --recursive
cargo run -- --type xml --recursive
cargo run -- --type yang --detailed
```

## 特色功能

### 🎨 专业的表格输出
工具采用类似tokei的表格格式，提供清晰美观的统计结果展示。

### 🌈 彩色输出
- **XML文件**: 黄色高亮
- **YANG文件**: 青色高亮  
- **统计数据**: 不同颜色区分不同类型的数据

### 🚀 用户友好
- 无需强制指定路径，默认使用当前目录
- 支持递归搜索子目录（需要明确指定）
- 详细模式显示每个文件的具体统计信息
- 英文表头，国际化友好

### 📊 准确统计
- 精确统计文件总行数
- 统计文件总字符数
- 识别并统计注释行
- 统计空白行数

## 许可证

[添加你的许可证信息]
