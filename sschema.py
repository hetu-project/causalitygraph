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
    url = "http://144.126.138.135:8080/alter"

    
    # 请求头
    headers = {
        "Content-Type": "application/json"
    }
    print(url)
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
sig: string .
lamport_id: string @index(hash) .
eth_address: string @index(hash) .
project_name: string @index(hash) .
eth_sig: string .
twitter_id: string @index(hash) .
pubkey: string @index(hash) .
created_by: uid .
author: uid @reverse .
posts: [uid] @reverse .

reply: uid .  # 单一关系，指向被提及的帖子
replyed_by: [uid] @reverse .  # 列表关系，指向提及当前帖子的帖子
root: uid .  # 单一关系，指向根帖子
child: [uid] @reverse .  # 列表关系，指向当前帖子的子帖子

inviter: string .
invitee: string .
project_info: string .
invite: [uid] @reverse .
label: string .
user_count: int .
event_count: int .
records_count: int .
event_type: [string] .

mention_p: [uid] @reverse .
mentioned_by: [uid] @reverse .
tags: [uid] @reverse .

participates_in: [uid] @reverse .
votes: [uid] @reverse .
create_votes: [uid] @reverse .

follow: [uid] @reverse .
retweet: [uid] @reverse .


platform: string @index(hash) .
post_id: string @index(hash) .

vote_title: string @index(hash) .
vote_options: string .

type User {
  id
  pubkey
  lamport_id
  twitter_id
  eth_address
  eth_sig
  sig
  posts
  mentioned_by
  invite
  follow
  retweet
  participates_in
  event_type
  votes
  create_votes
}

type Tag {
  id
  tag_content
  posts
}

type Vote {
  id
  vote_title
  content
  vote_options
  created_at
}

type Invite {
  id
  pubkey
  created_at
  kind
  inviter
  invitee
  content
  project_info
  sig
}

type Project {
  id
  content
  created_by
  created_at
  project_name
  user_count
  event_count
  records_count
  event_type
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
  platform
  post_id
  mention_p
  tags
  sig
}
'''


result = alter_schema(schema_definition)
print(result)