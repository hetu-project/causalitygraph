import requests
import json

# 定义查询
query = {
    "query": """
    {
        users(func: uid(0x2867)) {
            uid
            dgraph.type
            pubkey
            name
            about
            picture
            nip05
            website
            lud16
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
    users = result.get("data", {}).get("users", [])
    if users:
        print("查询成功，返回的数据:")
        print(json.dumps(users, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到数据。")
except Exception as e:
    print(f"查询失败: {e}")