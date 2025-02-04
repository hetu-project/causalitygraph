import requests
import json

url = "http://144.126.138.135:8080/mutate?commitNow=true"
headers = {"Content-Type": "application/json"}


import requests
import json

# 定义 URL 和请求头
url = "http://144.126.138.135:8080/mutate?commitNow=true"
headers = {"Content-Type": "application/json"}

# 查询 0x283f 的入边
query = {
  "query": """
  {
    node(func: uid(0x283f)) {
      uid
      ~User {  # 查询指向 0x283f 的 User 边
        uid
      }
      ~create_votes {  # 查询指向 0x283f 的 create_votes 边
        uid
      }
    }
  }
  """
}

try:
    # 查询入边
    response = requests.post(url, headers=headers, data=json.dumps(query))
    response.raise_for_status()
    result = response.json()
    print("查询结果:")
    print(json.dumps(result, indent=2, ensure_ascii=False))

    # 解析查询结果
    create_votes_edges = result["data"]["queries"]["node"][0]["~create_votes"]

    # 构造删除操作
    delete_queries = []
    for edge in create_votes_edges:
        delete_queries.append({
            "uid": edge["uid"],  # 边的源节点
            "create_votes": {"uid": "0x283f"}  # 删除指向 0x283f 的 create_votes 边
        })

    # 发送删除请求
    if delete_queries:
        delete_query = {"delete": delete_queries}
        response = requests.post(url, headers=headers, data=json.dumps(delete_query))
        response.raise_for_status()
        result = response.json()
        print("删除操作结果:")
        print(json.dumps(result, indent=2, ensure_ascii=False))
    else:
        print("没有需要删除的边。")
except Exception as e:
    print(f"操作失败: {e}")

exit()


query = {
  "query": """
  {
    node(func: uid(0x283f)) {
      uid
      ~User {  # 查询指向 0x283f 的 User 边
        uid
      }
      ~create_votes {  # 查询指向 0x283f 的 Vote 边
        uid
      }
    }
  }
  """
}

response = requests.post(url, headers=headers, data=json.dumps(query))
result = response.json()
print("查询结果:")
print(json.dumps(result, indent=2, ensure_ascii=False))
exit()

# 定义删除操作
delete_query = {
  "delete": [
    {
      "uid": "0x1",  # 替换为你想删除的节点的 uid
      "edges": {
        # 这里列出你想删除的边类型
        "Vote": {}   # 替换为实际的边类型
      }
    }
  ]
}

# 发送 HTTP 请求


try:
    response = requests.post(url, headers=headers, data=json.dumps(delete_query))
    response.raise_for_status()  # 检查请求是否成功
    result = response.json()
    print("删除操作结果:")
    print(json.dumps(result, indent=2, ensure_ascii=False))
except Exception as e:
    print(f"删除操作失败: {e}")

exit()