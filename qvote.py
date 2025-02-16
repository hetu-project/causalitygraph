import requests
import json

query = {
    "query": """
    {
        Vote(func: uid(0x2840)) {
            uid
            dgraph.type
            id
            vote_title
            content
            vote_options
            created_at
            ~create_votes{
                uid
                lamport_id
            }
        }
    }
    """
}

url = "http://144.126.138.135:8080/query"
url = "http://212.56.40.235:8080/query"

headers = {"Content-Type": "application/json"}

print(query)
try:
    response = requests.post(url, headers=headers, data=json.dumps(query))
    response.raise_for_status() 
    result = response.json()
    print(result)
    event = result.get("data", {}).get("Vote", [])
    if event:        
        print(json.dumps(event, indent=2, ensure_ascii=False))
    else:
        print("no post")
except Exception as e:
    print(f"error: {e}")



# exit()
query = {
    "query": """
    {
        user(func: type(Vote)) {
            uid
            dgraph.type
            id
            vote_title
            content
            vote_options
            created_at
            ~create_votes{
                uid
                lamport_id
            }
        }
    }
    """
}

url = "http://144.126.138.135:8080/query"
url = "http://212.56.40.235:8080/query"

headers = {"Content-Type": "application/json"}

try:
    response = requests.post(url, headers=headers, data=json.dumps(query))
    response.raise_for_status()
    result = response.json()
    print(result)
    edges = []
    users = []
    nodes = result.get("data", {}).get("user", [])
    if nodes:
        for node in nodes:
            uid = node.get("uid")
            node_type = node.get("dgraph.type")[0]
            pubkey = node.get("pubkey")
            created_at = node.get('created_at')
            content = node.get('content')

            one_user = {'id':uid, 'type':node_type, 'pubkey':pubkey, 'created_at':created_at, 'content':content }
            users.append(one_user)
            print(f"node UID: {uid}, type: {node_type}")
            for predicate, value in node.items():
                if predicate == 'invite':
                    for one_value in value:
                        print(f"边: {uid} -> {one_value['uid']}")
                        one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'type':'post'}
                        edges.append(one_edge)
        print(edges)
        print(users)
        # print(json.dumps(nodes, indent=2, ensure_ascii=False))
    else:
        print("no post")
except Exception as e:
    print(f"error: {e}")
