import requests


def alter_schema(schema_definition):
    url = "http://localhost:8080/alter"
    url = "http://144.126.138.135:8080/alter"

    
    headers = {
        "Content-Type": "application/json"
    }
    print(url)
    payload = schema_definition
    
    response = requests.post(url, headers=headers, data=payload)
    
    return response.json()


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

reply: uid . 
replyed_by: [uid] @reverse . 
root: uid . 
child: [uid] @reverse . 

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