import requests
import json

# 定义查询
query = {
  "query": """
  {
    nodes(func: has(dgraph.type)) {
      uid
      dgraph.type
      expand(_all_) {
        uid
        dgraph.type
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
    event = result.get("data", {}).get("nodes", [])
    if event:
        print("查询成功，返回的帖子及其提到的用户数据:")
        print(json.dumps(event, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到帖子数据。")
except Exception as e:
    print(f"查询失败: {e}")



import requests
import json

# 定义删除操作
delete_query = {
  "delete": [
    {
      "uid": "0x2876"  # 替换为你想删除的节点的 uid
    }
  ]
}

# 发送 HTTP 请求
url = "http://144.126.138.135:8080/mutate?commitNow=true"
headers = {"Content-Type": "application/json"}

try:
    response = requests.post(url, headers=headers, data=json.dumps(delete_query))
    response.raise_for_status()  # 检查请求是否成功
    result = response.json()
    print("删除操作结果:")
    print(json.dumps(result, indent=2, ensure_ascii=False))
except Exception as e:
    print(f"删除操作失败: {e}")

exit()
query = {
    "query": """
    {
        event(func: type(User)) {  # 替换为实际的帖子 UID
            uid
            dgraph.type
            id
            content
            mentions {
                uid
                dgraph.type
                pubkey
                name
                about
                picture
                nip05
                website
                lud16
                posts
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
    event = result.get("data", {}).get("event", [])
    if event:
        print("查询成功，返回的帖子及其提到的用户数据:")
        print(json.dumps(event, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到帖子数据。")
except Exception as e:
    print(f"查询失败: {e}")
