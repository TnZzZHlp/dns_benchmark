#!/bin/bash

echo "=== DNS压力测试工具演示 ==="
echo

echo "1. 基础测试 - 查询google.com，3秒，10 QPS"
cargo run -- -t 8.8.8.8:53 -d google.com -T 3 -r 10
echo

echo "2. 中等压力测试 - 查询example.com，5秒，50 QPS"
cargo run -- -t 8.8.8.8:53 -d example.com -T 5 -r 50
echo

echo "3. 高压力测试 - 查询cloudflare-dns.com，10秒，100 QPS"
cargo run -- -t 1.1.1.1:53 -d cloudflare-dns.com -T 10 -r 100
echo

echo "4. 自定义DNS服务器测试 - 使用阿里DNS，查询taobao.com"
cargo run -- -t 223.5.5.5:53 -d taobao.com -T 5 -r 30
echo

echo "5. 短时间高频率测试 - 查询bing.com，2秒，200 QPS"
cargo run -- -t 8.8.8.8:53 -d bing.com -T 2 -r 200
echo

echo "6. 长时间稳定测试 - 查询github.com，30秒，20 QPS"
echo "注意：这个测试会运行30秒..."
cargo run -- -t 8.8.8.8:53 -d github.com -T 30 -r 20
echo

echo "测试完成！"