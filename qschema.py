import requests
import json

headers = {"Content-Type": "application/json"}


query = {
    "query": """
    {
        getGQLSchemea {
            schema
        }
    },
    "variables":{}
    """
}

query = {
    "query": """
    {
        getGQLSchema {
            schema
        }
    }
    """
}

# 发送 HTTP 请求
url = "http://212.56.40.235:8080/admin"
headers = {"Content-Type": "application/json"}

try:
    response = requests.post(url, headers=headers, data=json.dumps(query))
    response.raise_for_status()  # 检查请求是否成功
    result = response.json()
    print(result)
    # 解析查询结果
    edges = result.get("data", {}).get("schema", [])
    if edges:
        print("查询成功，返回的边数据:")
        print(json.dumps(edges, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到边数据。")
except Exception as e:
    print(f"查询失败: {e}")



import requests
import json

headers = {"Content-Type": "application/json"}


query = {
    "query": """
    {
        getGQLSchemea {
            schema
        }
    },
    "variables":{}
    """
}

query = r"""
        schema{
            type
            predicate
        }
    """

# query = r"""
#     schema(type: [User, Post]) {
#     type
#     }

# """

# 发送 HTTP 请求
url = "http://localhost:8080/query"
headers = {"Content-Type": "application/dql"}

try:
    response = requests.post(url, headers=headers, data=query)
    response.raise_for_status()  # 检查请求是否成功
    result = response.json()
    print(result)
    # 解析查询结果
    edges = result.get("data", {}).get("schema", [])
    if edges:
        print("查询成功，返回的schema数据:")
        print(json.dumps(edges, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到schema数据。")
    edges = result.get("data", {}).get("types", [])
    if edges:
        print("查询成功，返回的types数据:")
        print(json.dumps(edges, indent=2, ensure_ascii=False))
    else:
        print("查询成功，但未找到types数据。")
except Exception as e:
    print(f"查询失败: {e}")


exit()

import requests

def get_full_field_info():
    """
    获取所有谓词的详细信息。
    """
    # 查询语句
    query = r"""
    schema {
      predicate
      type
    }
    """

    # 发送 HTTP 请求
    url = "http://localhost:8080/query"
    headers = {"Content-Type": "application/dql"}
    response = requests.post(url, headers=headers, data=query)

    # 返回结果
    return response.json()

# 调用函数获取谓词信息
schema_info = get_full_field_info()
print(schema_info)


def get_type_fields(schema_info, type_name):
    """
    从 Schema 信息中提取特定类型的字段信息。

    :param schema_info: Schema 查询结果
    :param type_name: 类型名称，例如 "User" 或 "Post"
    :return: 类型的字段信息
    """
    # 提取类型定义
    types = schema_info.get("data", {}).get("schema", [])
    for type_def in types:
        if type_def.get("type") == type_name:
            return type_def.get("fields", [])
    return []

# 获取所有谓词信息
schema_info = get_full_field_info()

# 获取 User 类型的字段信息
user_fields = get_type_fields(schema_info, "User")
print("User fields:", user_fields)

# 获取 Post 类型的字段信息
post_fields = get_type_fields(schema_info, "Post")
print("Post fields:", post_fields)

