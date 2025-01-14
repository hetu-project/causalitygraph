import requests
import json

# 定义查询
query = {
    "query": """
    {
        event(func: uid(0x2868)) {  # 替换为实际的帖子 UID
            uid
            dgraph.type
            id
            content
            has_tag {
                uid
                dgraph.type
                name
            }
        }
    }
    """
}

# 发送 HTTP 请求
url = "http://144.126.138.135:8080/query"
headers = {"Content-Type": "application/json"}

try:
    response = requests.post(url, headers=headers, data=json.dumps(query))
    response.raise_for_status()  # 检查请求是否成功
    result = response.json()
    print(result)
    # 解析查询结果
    event = result.get("data", {}).get("event", [])
    if event:
        print("查询成功，返回的帖子及其标签数据:")
        print(json.dumps(event, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到帖子数据。")
except Exception as e:
    print(f"查询失败: {e}")