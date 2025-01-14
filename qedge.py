import requests
import json

# 定义查询
query = {
  "query": """
  {
    events(func: type(User)) {
      uid
      dgraph.type
      posts {
        uid
        dgraph.type
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
    # 解析查询结果
    nodes = result.get("data", {}).get("events", [])
    if nodes:
        print("查询成功，返回的节点和边数据:")
        for node in nodes:
            uid = node.get("uid")
            node_type = node.get("dgraph.type")
            print(f"节点 UID: {uid}, 类型: {node_type}")
            for predicate, value in node.items():
                if predicate not in ["uid", "dgraph.type"]:
                    print(f"边: {predicate} -> {value}")
    else:
        print("查询成功，但未找到节点数据。")
except Exception as e:
    print(f"查询失败: {e}")

exit()


query = {
  "query": """
  schema {
    predicate
    type
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
    edges = result.get("data", {}).get("edges", [])
    if edges:
        print("查询成功，返回的边数据:")
        print(json.dumps(edges, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到边数据。")
except Exception as e:
    print(f"查询失败: {e}")