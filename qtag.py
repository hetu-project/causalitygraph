import requests
import json

# 定义查询
query = {
    "query": """
    {
        tags(func: uid(0x28c7)) {  # 查询类型为 Post 的节点
            uid
            dgraph.type
            id
            tag_content
            created_at
            kind
            posts {
                uid
                dgraph.type
                content
            }
        }
    }
    """
}

# 发送 HTTP 请求
url = "http://144.126.138.135:8080/query"
url = "http://212.56.40.235:8080/query"

headers = {"Content-Type": "application/json"}

try:
    response = requests.post(url, headers=headers, data=json.dumps(query))
    response.raise_for_status()  # 检查请求是否成功
    result = response.json()
    print(result)
    # 解析查询结果
    event = result.get("data", {}).get("tags", [])
    if event:
        print("查询成功，返回的帖子及其提到的用户数据:")
        print(json.dumps(event, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到帖子数据。")
except Exception as e:
    print(f"查询失败: {e}")

# exit()
# 定义查询
query = {
    "query": """
    {
        tag(func: eq(tag_content, "staySAIF")) {
            uid
            dgraph.type
            id
            tag_content
            created_at
            kind
            posts {
                uid
                dgraph.type
                content
            }
        }
    }
    """
}

# 发送 HTTP 请求
url = "http://144.126.138.135:8080/query"
url = "http://212.56.40.235:8080/query"

headers = {"Content-Type": "application/json"}

try:
    response = requests.post(url, headers=headers, data=json.dumps(query))
    response.raise_for_status()  # 检查请求是否成功
    result = response.json()
    print(result)
    # 解析查询结果
    event = result.get("data", {}).get("tag", [])
    if event:
        print("查询成功，返回的帖子及其提到的用户数据:")
        print(json.dumps(event, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到帖子数据。")
except Exception as e:
    print(f"查询失败: {e}")
