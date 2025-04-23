#!/bin/bash

set -e

# 参数
NODES=3
BINARY="./orbitdb"
DATA_DIR="~/data"
LOG_DIR="~/logs"
START_PORT=4001
API_PORT=5001  # 可根据你的实现调整
WAIT_DB=5

# 清理旧数据
rm -rf "$DATA_DIR" "$LOG_DIR"
mkdir -p "$LOG_DIR"

# 1. 启动第一个节点（新建数据库）
echo "=== 启动 node1，生成新数据库... ==="
"$BINARY" -data "${DATA_DIR}/node1" -listen "/ip4/0.0.0.0/tcp/$START_PORT" > "${LOG_DIR}/node1.log" 2>&1 &
NODE1_PID=$!
sleep $WAIT_DB

# 2. 提取数据库地址
DB_ADDR=$(grep -m 1 "Database created with address:" "${LOG_DIR}/node1.log" | awk '{print $NF}')
if [ -z "$DB_ADDR" ]; then
    echo "未成功获取数据库地址，检查 node1 日志：${LOG_DIR}/node1.log"
    kill $NODE1_PID
    exit 1
fi
echo "发现数据库地址：$DB_ADDR"

# 3. 启动第2、第3节点，加入同一数据库
for n in 2 3; do
    port=$((START_PORT + n - 1))
    echo "=== 启动 node$n，连接数据库 $DB_ADDR... ==="
    "$BINARY" -data "${DATA_DIR}/node$n" -listen "/ip4/0.0.0.0/tcp/$port" -db "$DB_ADDR" > "${LOG_DIR}/node${n}.log" 2>&1 &
    eval NODE${n}_PID=\$!
    sleep 3
done

# 4. 打印所有节点日志路径
echo "所有节点已启动，日志路径如下："
for n in 1 2 3; do
    echo "node$n: ${LOG_DIR}/node${n}.log"
done

echo ""
echo "可通过如下命令实时查看日志（例）："
echo "    tail -f ${LOG_DIR}/node1.log"
echo "    tail -f ${LOG_DIR}/node2.log"
echo "    tail -f ${LOG_DIR}/node3.log"
echo ""
echo "稍等 10 秒后自动采集各节点数据库内容进行一致性检查。"

sleep 10

echo "=== 数据库内容采集与对比（如含 DB state 或 Fetched file contents） ==="
for n in 1 2 3; do
    echo "--- node$n ---"
    grep -E "Final DB state|Fetched file contents|Added entry to OrbitDB" "${LOG_DIR}/node${n}.log" || echo "(未捕获日志)"
    echo ""
done

echo "=== 若各节点内容相同，说明同步与一致性良好。按 Ctrl+C 杀掉脚本并清理所有节点。==="

trap "kill $NODE1_PID $NODE2_PID $NODE3_PID; echo '已终止所有节点。'; exit 0" INT
wait