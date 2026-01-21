# DNS压力测试工具（UDP）

一个使用Rust编写的DNS压力测试工具，专门用于测试DNS服务器的UDP性能。

## 功能特点

- 纯UDP DNS查询压力测试
- 支持自定义查询速率（QPS）
- 实时进度显示和统计信息
- 多线程并发处理
- 详细的性能统计报告
- 命令行参数配置

## 安装

```bash
git clone <repository-url>
cd dns_benchmark
cargo build --release
```

## 使用方法

### 基本命令

```bash
# 显示帮助信息
cargo run -- --help

# 基本测试：查询google.com，10秒，100 QPS
cargo run -- -t 8.8.8.8:53 -d google.com -T 10 -r 100

# 使用自定义DNS服务器
cargo run -- -t 1.1.1.1:53 -d example.com -T 30 -r 50

# 调整并发工作线程数
cargo run -- -t 8.8.8.8:53 -d bing.com -T 15 -r 200 -w 20
```

### 命令行参数

```
-t, --target <TARGET>      DNS服务器地址（主机:端口） [默认: 192.168.2.1:53]
-d, --domain <DOMAIN>      要查询的域名 [默认: example.com]
-c, --count <COUNT>        要发送的总查询数 [默认: 100000]
-r, --rate <RATE>          每秒查询数（QPS） [默认: 100]
-o, --timeout <TIMEOUT>    每个查询的超时时间（秒） [默认: 5]
-w, --workers <WORKERS>    并发工作线程数 [默认: 10]
-m, --mode <MODE>          测试模式：same-domain 或 random-subdomain [默认: same-domain]
```

### 示例测试场景

1. **相同域名测试（默认模式）**
   ```bash
   # 测试同一个域名
   cargo run -- -t 8.8.8.8:53 -d google.com -c 6000 -r 100 -m same-domain
   ```

2. **随机子域名测试**
   ```bash
   # 测试随机子域名（如 abcdef1234.google.com, ghijkl5678.google.com等）
   cargo run -- -t 1.1.1.1:53 -d cloudflare.com -c 60000 -r 500 -m random-subdomain
   ```

3. **短时间爆发测试**
   ```bash
   # 使用随机子域名进行高并发测试
   cargo run -- -t 9.9.9.9:53 -d quad9.net -c 10000 -r 1000 -m random-subdomain
   ```

4. **稳定性测试**
   ```bash
   # 长时间相同域名测试
   cargo run -- -t 223.5.5.5:53 -d aliyun.com -c 15000 -r 50 -m same-domain
   ```

## 输出示例

```
DNS Benchmark Tool
Target: 8.8.8.8:53
Domain: google.com
Total queries: 6000
Rate: 100 QPS
Timeout: 5 seconds
Workers: 10
----------------------------------------
⠙ [00:05] [#################>-----------------] 50% (QPS: 98.3, Success: 99.5%)

=== Benchmark Results ===
Duration: 60.02s
Total requests: 6000
Successful responses: 5985
Failed responses: 15
Requests per second: 99.96
Success rate: 99.75%
```

## 技术细节

### DNS报文格式
- 支持标准DNS查询报文（RFC 1035）
- 使用随机查询ID
- 支持A记录查询
- IN类查询
- 支持两种测试模式：
  - 相同域名：所有查询使用同一个域名
  - 随机子域名：为每个查询生成随机子域名前缀

### 性能特性
- 异步I/O（使用Tokio）
- 无锁原子计数器统计
- 实时进度显示
- 错误处理和重试

### 安全考虑
- 仅发送DNS查询请求
- 不进行任何恶意活动
- 可配置速率限制
- 超时机制防止阻塞

## 构建发布版本

```bash
# 调试版本
cargo build

# 发布版本（优化性能）
cargo build --release

# 安装到系统
cargo install --path .
```

## 故障排除

### 常见问题

1. **连接被拒绝**
   - 检查DNS服务器地址和端口
   - 确保网络连接正常
   - 验证防火墙设置

2. **成功率低**
   - 降低查询速率（-r参数）
   - 增加超时时间
   - 检查DNS服务器状态

3. **性能问题**
   - 调整工作线程数（-w参数）
   - 检查系统资源限制
   - 使用release版本

### 调试信息

```bash
# 启用详细日志
RUST_LOG=debug cargo run -- [参数]

# 查看网络连接
netstat -an | grep :53
```

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！