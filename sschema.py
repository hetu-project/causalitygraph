import requests

"""
必须在这定义【】
posts: [uid] @reverse .
定义post列表
否则它根本不能正确同步posts列表

"""


def alter_schema(schema_definition):
    """
    修改 Dgraph 的 Schema。

    :param schema_definition: Schema 定义（字符串形式）。
    :return: 请求的响应结果。
    """
    # Dgraph 的 /alter 端点
    url = "http://localhost:8080/alter"
    
    # 请求头
    headers = {
        "Content-Type": "application/json"
    }
    
    # 请求体
    payload = schema_definition
    
    # 发送 POST 请求
    response = requests.post(url, headers=headers, data=payload)
    
    # 返回响应结果
    return response.json()

# 示例 Schema 定义
schema_definition = '''
id: string @index(hash) .
content: string .
tag_content: string @index(hash) .
created_at: datetime .
kind: string .
pubkey: string @index(hash) .

author: uid @reverse .
posts: [uid] @reverse .

reply: uid .  # 单一关系，指向被提及的帖子
replyed_by: [uid] @reverse .  # 列表关系，指向提及当前帖子的帖子
root: uid .  # 单一关系，指向根帖子
child: [uid] @reverse .  # 列表关系，指向当前帖子的子帖子

mention_p: [uid] @reverse .
mentioned_by: [uid] @reverse .
tags: [uid] @reverse .

type User {
  pubkey
  posts
  mentioned_by
}

type Tag {
  tag_content
  posts
}

type Post {
  id
  content
  created_at
  kind
  author
  reply
  replyed_by
  root
  child
  mention_p
  tags
}
'''


result = alter_schema(schema_definition)
print(result)