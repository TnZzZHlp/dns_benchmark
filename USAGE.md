# 快速使用指南

## 安装

```bash
# 克隆仓库
git clone <repository-url>
cd dns_benchmark

# 构建工具
cargo build --release

# 工具位置：target/release/dns_benchmark
```

## 基本使用

### 1. 测试公共DNS服务器

```bash
# 测试Google DNS
./target/release/dns_benchmark -t 8.8.8.8:53 -d google.com -T 10 -r 100

# 测试Cloudflare DNS
./target/release/dns_benchmark -t 1.1.1.1:53 -d cloudflare.com -T 10 -r 100

# 测试Quad9 DNS
./target/release/dns_benchmark -t 9.9.9.9:53 -d quad9.net -T 10 -r 100
```

### 2. 测试本地DNS服务器

```bash
# 测试本地路由器DNS
./target/release/dns_benchmark -t 192.168.1.1:53 -d localhost -T 30 -r 50

# 测试公司内部DNS
./target/release/dns_benchmark -t 10.0.0.1:53 -d company.com -T 60 -r 100
```

### 3. 性能基准测试

```bash
# 低负载测试（稳定性）
./target/release/dns_benchmark -T 300 -r 10

# 中等负载测试（日常使用）
./target/release/dns_benchmark -T 120 -r 100

# 高负载测试（压力测试）
./target/release/dns_benchmark -T 60 -r 500

# 极限负载测试（性能极限）
./target/release/dns_benchmark -T 30 -r 1000
```

## 实用脚本

### 批量测试脚本

```bash
#!/bin/bash
# test_multiple_dns.sh

SERVERS=("8.8.8.8:53" "1.1.1.1:53" "9.9.9.9:53" "223.5.5.5:53")
DOMAIN="example.com"
DURATION=10
RATE=100

for server in "${SERVERS[@]}"; do
    echo "测试DNS服务器: $server"
    ./target/release/dns_benchmark -t "$server" -d "$DOMAIN" -T "$DURATION" -r "$RATE"
    echo "----------------------------------------"
done
```

### 结果分析脚本

```bash
#!/bin/bash
# analyze_results.sh

echo "DNS压力测试结果分析"
echo "=================="
echo

# 运行测试并捕获输出
RESULTS=$(./target/release/dns_benchmark -t 8.8.8.8:53 -d google.com -T 5 -r 50)

# 提取关键指标
echo "$RESULTS" | grep -E "(Requests per second|Success rate|Total requests)"

# 生成建议
echo
echo "性能建议:"
echo "1. 如果成功率低于95%，考虑降低查询频率"
echo "2. 如果QPS低于预期，检查网络延迟"
echo "3. 如果有大量失败，检查DNS服务器状态"
```

## 故障排除

### 问题：连接超时或失败
```bash
# 增加超时时间（默认5秒）
# 修改源代码中的DnsClient超时设置
```

### 问题：系统资源不足
```bash
# 减少工作线程数
./target/release/dns_benchmark -w 5 -r 50

# 降低查询频率
./target/release/dns_benchmark -r 10 -T 30
```

### 问题：权限不足
```bash
# 使用sudo（如果需要）
sudo ./target/release/dns_benchmark -t 8.8.8.8:53 -d test.com -T 10 -r 100
```

## 最佳实践

1. **开始前测试**：先用低频率测试确认连接正常
2. **逐步增加负载**：从低到高逐步增加查询频率
3. **监控系统资源**：使用top或htop监控CPU和内存使用
4. **记录结果**：保存测试结果用于比较和分析
5. **遵守使用政策**：不要对未经授权的服务器进行压力测试

## 常用命令速查

```bash
# 帮助信息
./target/release/dns_benchmark --help

# 版本信息
./target/release/dns_benchmark --version

# 快速测试（推荐新用户）
./target/release/dns_benchmark -T 3 -r 10

# 完整参数示例
./target/release/dns_benchmark \
    -t 8.8.8.8:53 \
    -d example.com \
    -T 60 \
    -r 200 \
    -w 20 \
    -c 12000
```