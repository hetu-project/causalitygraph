import requests

# GraphQL 查询
query = """
{
    users {
        uid
    }
}
"""

# 定义请求的 URL 和 headers
url = "http://localhost:5005/graphql"
headers = {"Content-Type": "application/json"}

# 发送 POST 请求
response = requests.post(url, headers=headers, json={"query": query})

# 检查响应状态码
if response.status_code == 200:
    # 打印响应数据
    print("GraphQL 查询成功！")
    print("响应数据：")
    print(response.json())
else:
    print(f"GraphQL 查询失败，状态码：{response.status_code}")
    print("错误信息：")
    print(response.text)