import json
import threading
from flask import Flask, request, jsonify
from ariadne import QueryType, make_executable_schema, graphql_sync
import pydgraph
import websocket as websocket_client
import requests
import uuid
from flask_cors import CORS
from config import *



def query_user(pubkey):
    query = {
        "query": """
        {
            user(func: eq(pubkey, "%s")) {
                uid
                dgraph.type
                pubkey
                posts {
                    uid
                }
                mentioned_by {
                    uid
                }
            }
        }
        """ % pubkey
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status()
        result = response.json()
        user = result.get("data", {}).get("user", [])
        if user:
            return user[0] 
        else:
            return None
    except Exception as e:
        print(f"find User error: {e}")
        return None

def query_user_by_account(platform, account):
    query = {
        "query": """
        {
            user(func: eq(account, "%s")) @filter(eq(platform, "%s")) {
                uid
                dgraph.type
                pubkey
                posts {
                    uid
                }
                mentioned_by {
                    uid
                }
            }
        }
        """ % (account, platform)
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status()
        result = response.json()
        user = result.get("data", {}).get("user", [])
        if user:
            return user[0] 
        else:
            return None 
    except Exception as e:
        print(f"get User error: {e}")
        return None

def query_post_by_account(platform, post_id):
    query = {
        "query": """
        {
            post(func: eq(post_id, "%s")) @filter(eq(platform, "%s")) {
                uid
                dgraph.type
                id
            }
        }
        """ % (post_id, platform)
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status()
        result = response.json()
        user = result.get("data", {}).get("post", [])
        if user:
            return user[0] 
        else:
            return None 
    except Exception as e:
        print(f"get post error: {e}")
        return None


def query_project(project_name):
    query = {
        "query": """
        {
            project(func: eq(project_name, "%s")) {
                uid
                dgraph.type

            }
        }
        """ % project_name
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status() 
        result = response.json()
        user = result.get("data", {}).get("project", [])
        if user:
            return user[0] 
        else:
            return None
    except Exception as e:
        print(f"get project error: {e}")
        return None


def query_post(post_id):
    query = {
        "query": """
        {
            post(func: eq(id, "%s")) {
                uid
                dgraph.type
                id
            }
        }
        """ % post_id
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status()
        result = response.json()
        post = result.get("data", {}).get("post", [])
        if post:
            return post[0]
        else:
            return None 
    except Exception as e:
        print(f"get post error: {e}")
        return None

def query_lamport(lamport_id):
    query = {
        "query": """
        {
            user(func: eq(lamport_id, "%s")) {
                uid
                dgraph.type
                id
            }
        }
        """ % lamport_id
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status()
        result = response.json()
        user = result.get("data", {}).get("user", [])
        if user:
            return user[0]
        else:
            return None 
    except Exception as e:
        print(f"get user error: {e}")
        return None


def query_id(id):
    query = {
        "query": """
        {
            event(func: eq(id, "%s")) {
                uid
                dgraph.type
                id
            }
        }
        """ % id
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status() 
        result = response.json()
        event = result.get("data", {}).get("event", [])
        if event:
            return event[0]
        else:
            return None 
    except Exception as e:
        print(f"get id: {e}")
        return None

def query_tag(content):
    query = {
        "query": """
        {
            tag(func: eq(tag_content, "%s")) {
                uid
                dgraph.type
                tag_content
                posts {
                    uid
                }
            }
        }
        """ % content
    }

    try:
        response = requests.post(query_url, headers=headers, data=json.dumps(query))
        response.raise_for_status() 
        result = response.json()

        tag = result.get("data", {}).get("tag", [])
        print(f'tag result :\n {tag}')
        if tag:
            return tag[0]
        else:
            return None  
    except Exception as e:
        print(f"query tag  error: {e}")
        return None

def insert_dgraph(mutation, info=''):
    mutation_json = json.dumps(mutation)

    # try:
    if True:
        response = requests.post(mutation_url, headers=headers, data=mutation_json)
        response.raise_for_status() 
        result = response.json()
        print(result)
        uids = result.get("data", {}).get("uids", {})
        code = result.get("data", {}).get("code", {})

        if uids :
            print(f"insert {info} success, return UID:", uids)
        elif code == 'Success':
            print("change success")
        else:
            print(f"insert {info} failed \n {result}")

    # except Exception as e:
    #     print(f"insert {info} failed error: {e}")



client_stub = pydgraph.DgraphClientStub(DGRAPH_URI)
client = pydgraph.DgraphClient(client_stub)


# schema = make_executable_schema(type_defs, query)



def on_message(ws, message):
    # try:
    if True:
        data = json.loads(message)
        if data[0] == 'EVENT':
            _, describe_id, event = data
            # print("Received event:", event)
            # print("Received data:", data)
            print(data)


            save_event_to_dgraph(describe_id,event)
    # except Exception as e:
    #     print("Error parsing message:", e)

def save_event_to_dgraph(describe_id, event):
    if event["kind"] == 0: 
        mutation = {
            "set": [
                {
                    "uid": "_:user",
                    "dgraph.type": "User",
                    "pubkey": event["pubkey"],
                    "name": event.get("name"),
                    "about": event.get("about"),
                    "picture": event.get("picture"),
                    "nip05": event.get("nip05"),
                    "website": event.get("website"),
                    "lud16": event.get("lud16")
                }
            ]
        }
        insert_dgraph(mutation)
    elif event["kind"] == 10:
        print(event["id"])
        post_uid = query_post(event["id"])
        if post_uid:
            print(f"post have been recorded, uid: {post_uid}")
            return  
        user_data = query_user(event["pubkey"])
        if user_data and "posts" in user_data:
            if isinstance(user_data["posts"], dict):
                user_data["posts"] = [user_data["posts"]]
        else:
            user_data = None 
        print('user data:\n', user_data)

        post_data = {
                    "uid": "_:post",
                    "dgraph.type": "Post",
                    "id": event["id"],
                    "content": event["content"],
                    "created_at": event["created_at"],
                    "kind": event["kind"],
                    "author": {
                        "uid": user_data['uid'] if user_data else "_:user",
                        "dgraph.type": "User",
                        "pubkey": event["pubkey"],
                        "posts": [{"uid": "_:post"}] + (
                            [{"uid": post["uid"]} for post in user_data["posts"]]
                            if user_data and "posts" in user_data
                            else []
                        ) 
                        }
                }

        for tag in event.get("tags", []):
            if tag[0] == 'e':
                post_id = tag[1]
                marker = tag[3] if len(tag) > 3 else None
                referenced_post = query_post(post_id)
                print(f'referenced_post\n {referenced_post}')
                if referenced_post and "child" in referenced_post and isinstance(referenced_post["child"], dict): 
                    referenced_post["child"] = [referenced_post["child"]]
                if referenced_post and "replyed_by" in referenced_post and isinstance(referenced_post["replyed_by"], dict): 
                    referenced_post["replyed_by"] = [referenced_post["replyed_by"]]
            
                post_data[marker] = {
                    "uid": referenced_post['uid'] if referenced_post else "_:referenced_post",
                    "dgraph.type": "Post",
                    "id": post_id,
                    "kind": 1,
                }
                if marker == "reply":
                    post_data[marker]['replyed_by'] = [{"uid": "_:post"}] + (
                            [{"uid": post["uid"]} for post in referenced_post["replyed_by"]]
                            if referenced_post and "replyed_by" in referenced_post
                            else []
                        ) 
                elif marker == 'root':
                    post_data[marker]['child'] = [{"uid": "_:post"}] + (
                            [{"uid": post["uid"]} for post in referenced_post["child"]]
                            if referenced_post and "child" in referenced_post
                            else []
                        ) 
                
            if tag[0] == "p":
                mentioned_pubkey = tag[1]
                print(f'mentioned_pubkey:\n{mentioned_pubkey}')
                mentioned_user_data = query_user(mentioned_pubkey)
                if mentioned_user_data and "mentioned_by" in mentioned_user_data:
                    if isinstance(mentioned_user_data["mentioned_by"], dict): 
                        mentioned_user_data["mentioned_by"] = [user_data["mentioned_by"]]
                else:
                    mentioned_user_data = None
                print(f'mentioned_user_data status :\n{mentioned_user_data}')

                post_data['mention_p'] = {
                        "uid": mentioned_user_data['uid'] if mentioned_user_data else "_:mentioned_user",
                        "dgraph.type": "User",
                        "pubkey": mentioned_pubkey,
                        "mentioned_by": [{"uid": "_:post"}] + (
                            [{"uid": mentioned_by["uid"]} for mentioned_by in mentioned_user_data["mentioned_by"]]
                            if mentioned_user_data and "mentioned_by" in mentioned_user_data
                            else []
                        ) 
                    }

            elif tag[0] == "t": 
                tag_name = tag[1]

                print(f'tag_name:\n{tag_name}')
                tag_data = query_tag(tag_name)
                if tag_data and "posts" in tag_data:
                    if isinstance(tag_data["posts"], dict):
                        tag_data["posts"] = [tag_data["posts"]]
                else:
                    tag_data = None 
                print(f'tag_name_data status :\n{tag_data}')

                post_data['tags'] = {
                        "uid": tag_data['uid'] if tag_data else "_:tag",
                        "dgraph.type": "Tag",
                        "tag_content": tag_name,
                        "posts": [{"uid": "_:post"}] + (
                            [{"uid": post["uid"]} for post in tag_data["posts"]]
                            if tag_data and "posts" in tag_data
                            else []
                        ) 
                    }
        mutation = {
            "set": [
                    post_data
                ]
            }
        print(post_data)
        insert_dgraph(mutation, info='post')
    elif event["kind"] == 3: 
        mutation = {
            "set": [
                {
                    "uid": "_:user",
                    "dgraph.type": "User",
                    "pubkey": event["pubkey"],
                    "follows": [
                        {
                            "uid": "_:followed_user",
                            "dgraph.type": "User",
                            "pubkey": follow
                        } for follow in event.get("tags", [])
                    ]
                }
            ]
        }
        insert_dgraph(mutation, info='follow')
    elif event["kind"] == 6: 
        id = event["id"]
        pubkey = event["pubkey"]
        
        # created_at = event["created_at"]
        for tag in event.get("tags", []):
            if tag[0] == 't':
                platform = tag[1]
            if tag[0] == 'account':
                account = tag[1]
            if tag[0] == 'post_id':
                post_id = tag[1]
            if tag[0] == 'created_at':
                created_at = tag[1]
        user_data = query_user_by_account(platform, account)
        post_data = query_post_by_account(platform, post_id)
        mutation = {
            "set": [
                {
                    "uid": user_data['uid'] if user_data else "_:user",
                    "dgraph.type": "User",
                    "pubkey": event["pubkey"],
                    "created_at": created_at,
                    "retweet": [{post_data['uid'] if post_data else "uid": "_:post"}] + (
                            [{"uid": post["uid"]} for post in user_data["retweet"]]
                            if user_data and "retweet" in user_data
                            else []
                        ) 
                }
            ]
        }
        insert_dgraph(mutation, info='retweet')

    elif event["kind"] == 2321:
        user_data = query_user(event["pubkey"])
        if user_data:
            print(f"user have been recorded, uid: {user_data}")
            return  
    
        for tag in event.get("tags", []):
            if tag[0] == 'LamportId':
                lamport_id = tag[1]
            if tag[0] == 'Twitter':
                twitter_id = tag[1]
        
        print(f"lamport_id: {lamport_id}\n twitter_id:{twitter_id}")
        mutation = {
            "set": [
                {
                    "uid": "_:user",
                    "dgraph.type": "User",
                    "pubkey": event["pubkey"],
                    "created_at": event.get("created_at"),
                    "content": event.get("content"),
                    "lamport_id": lamport_id,
                    "twitter_id": twitter_id,
                    "sig": event.get("sig"),
                    "lud16": event.get("lud16")
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)
    elif event["kind"] == 2322:
        user_data = query_user(event["pubkey"])
        if not user_data:
            print(f"user have not recorded, please Bind an account")
            return  
        uid = user_data['uid']
        print(uid)
        for tag in event.get("tags", []):
            if tag[0] == 'LamportId':
                lamport_id = tag[1]
            if tag[0] == 'Address':
                eth_address = tag[1]
            if tag[0] == 'sig':
                eth_sig = tag[1]
        mutation = {
            "set": [
                {
                    "uid": uid,
                    "dgraph.type": "User",
                    "pubkey": event["pubkey"],
                    "created_at": event.get("created_at"),
                    "content": event.get("content"),
                    "lamport_id": lamport_id,
                    "eth_address": eth_address,
                    "eth_sig": eth_sig,
                    "sig": event.get("sig"),
                    "lud16": event.get("lud16")
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)

    elif event["kind"] == 2323:
        invite_data = query_id(event["id"])
        if invite_data:
            print(f"invite event have created")
            return  

        mutation = {
            "set": [
                {
                    "uid": "_:invite",
                    "dgraph.type": "Invite",
                    "id": event["id"],
                    "created_at": event.get("created_at")
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)
        for tag in event.get("tags", []):
            if tag[0] == 'LamportId':
                inviter = tag[1]
            if tag[0] == 'p':
                project_name = tag[1]
            if tag[0] == 'invitee':
                invitee = tag[1]
        inviter_data = query_lamport(inviter)
        invitee_data = query_lamport(invitee)

        project_data = query_project(project_name)
        if not project_data:
            print(f"project event have not created!")
            return  

        print(inviter_data)
        mutation = {
            "set": [
                {
                    "uid": inviter_data['uid'] if inviter_data else "_:inviter",
                    "lamport_id": inviter,
                    "dgraph.type": "User",
                    "invite|facets": [{
                                "nostr_id": event["id"],  
                                "project_name": project_name,
                                "content": event.get("content"),
                                "created_at": event.get("created_at"),
                            }] + 
                                (
                                [invite for invite in invitee_data["invite|facets"]]
                                if invitee_data and "invite|facets" in invitee_data
                                else []
                            ),
                    "~invite|facets": [{
                                "nostr_id": event["id"],  
                                "project_name": project_name,
                                "content": event.get("content"),
                                "created_at": event.get("created_at"),
                            }] + 
                                (
                                [invite for invite in invitee_data["~invite|facets"]]
                                if invitee_data and "~invite|facets" in invitee_data
                                else []
                            ),
                    "invite": [
                        {
                            "uid": invitee_data['uid'] if invitee_data else "_:invitee",
                            "dgraph.type": "User",
                            "lamport_id": invitee,
                            "~invite|facets":{
                                "nostr_id": event["id"],  
                                "project_name": project_name,
                                "content": event.get("content"),
                                "created_at": event.get("created_at"),
                            },
                            "participates_in": [{"uid": project_data['uid']}] + (
                                [{"uid": participates_in["uid"]} for participates_in in invitee_data["participates_in"]]
                                if invitee_data and "participates_in" in invitee_data
                                else []),
                        }
                        ]
                         + (
                            [invite for invite in invitee_data["invite"]]
                            if invitee_data and "invite" in invitee_data
                            else []
                         )
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)
    
    elif event["kind"] == 2410:
        vote_data = query_id(event["id"])
        if vote_data:
            print(f"vote event have created")
            return  

        for tag in event.get("tags", []):
            if tag[0] == 'LamportID':
                created_voter = tag[1]
            if tag[0] == 'vote_id':
                vote_id = tag[1]
            if tag[0] == 'title':
                title = tag[1]
            if tag[0] == 'content':
                content = tag[1]
            if tag[0] == 'options':
                options = tag[1].split(',')
        created_voter_data = query_lamport(created_voter)
        print(created_voter_data)
        if not created_voter_data:
            print(f"vote creator have created")
            return     
        mutation = {
            "set": [
                {
                    "uid": "_:vote",
                    "dgraph.type": "Vote",
                    "id": event["id"],
                    "created_at": event.get("created_at"),
                    "vote_id": vote_id,
                    "vote_title": title,
                    "content": content,
                    "vote_options": options,
                },
                {
                    "uid": created_voter_data['uid'],
                    "dgraph.type": "User",
                    "create_votes": [{"uid": "_:vote"}] + (
                                [{"uid": create_vote["uid"]} for create_vote in created_voter_data["create_votes"]]
                                if created_voter_data and "create_votes" in created_voter_data
                                else []),
                    
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)
    
    elif event["kind"] == 24111:
        invite_data = query_id(event["id"])
        if invite_data:
            print(f"invite event have created")
            return  

        mutation = {
            "set": [
                {
                    "uid": "_:inviter",
                    "dgraph.type": "Invite",
                    "id": event["id"],
                    "created_at": event.get("created_at")
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)
        for tag in event.get("tags", []):
            if tag[0] == 'LamportId':
                inviter = tag[1]
            if tag[0] == 'p':
                project_name = tag[1]
            if tag[0] == 'invitee':
                invitee = tag[1]
        inviter_data = query_lamport(inviter)
        invitee_data = query_lamport(invitee)

        project_data = query_project(project_name)
        if not project_data:
            print(f"project event have not created!")
            return  

        print(inviter_data)
        mutation = {
            "set": [
                {
                    "uid": inviter_data['uid']  if inviter_data else "_:inviter",
                    "lamport_id": inviter,
                    "dgraph.type": "User",
                    "invite|facets": [{
                                "nostr_id": event["id"],  
                                "project_name": project_name,
                                "content": event.get("content"),
                                "created_at": event.get("created_at"),
                            }] + 
                                (
                                [invite for invite in invitee_data["invite|facets"]]
                                if invitee_data and "invite|facets" in invitee_data
                                else []
                            ),
                    "~invite|facets": [{
                                "nostr_id": event["id"],  
                                "project_name": project_name,
                                "content": event.get("content"),
                                "created_at": event.get("created_at"),
                            }] + 
                                (
                                [invite for invite in invitee_data["~invite|facets"]]
                                if invitee_data and "~invite|facets" in invitee_data
                                else []
                            ),
                    "invite": [
                        {
                            "uid": invitee_data['uid'] if invitee_data else "_:invitee",
                            "dgraph.type": "User",
                            "lamport_id": invitee,
                            "~invite|facets":{
                                "nostr_id": event["id"],  
                                "project_name": project_name,
                                "content": event.get("content"),
                                "created_at": event.get("created_at"),
                            },
                            "participates_in": [{"uid": project_data['uid']}] + (
                                [{"uid": participates_in["uid"]} for participates_in in invitee_data["participates_in"]]
                                if invitee_data and "participates_in" in invitee_data
                                else []),
                        }
                        ]
                         + (
                            [invite for invite in invitee_data["invite"]]
                            if invitee_data and "invite" in invitee_data
                            else []
                         )
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)
    

    
    
    elif event['kind']== 30050:

        project_data = query_id(event["id"])
        if project_data:
            print(f"project event have created")
            return  
        user_data = query_user(event["pubkey"])
        if not user_data:
            print(f"User with pubkey {event['pubkey']} not found. Please bind an account.")
            return

        uid = user_data['uid']
        print(event)
        project_data = {
            "id": event["id"],
            "content": event.get("content"),
            "created_at": event.get("created_at"),
            "kind": event.get("kind"),
            "sig": event.get("sig"),
            "pubkey": event["pubkey"],
        }
        print('11111')
        for tag in event.get("tags", []):
            if tag[0] == "project_name":
                project_data["project_name"] = tag[1]
            elif tag[0] == "user_count":
                project_data["user_count"] = int(tag[1])
            elif tag[0] == "event_count":
                project_data["records_count"] = int(tag[1]) 
            elif tag[0] == "records_count":
                project_data["event_count"] = int(tag[1]) 
            elif tag[0] == "event_type":
                if "event_type" not in project_data:
                    project_data["event_type"] = []
                project_data["event_type"].append(tag[1])
        print('2222')


        mutation = {
            "set": [
                {
                    "uid": "_:project",
                    "dgraph.type": "Project",
                    "kind": project_data["kind"],
                    "project_name": project_data["project_name"],
                    "created_by": {
                        "uid": uid
                    },
                    "created_at": event.get("created_at"),
                    "content": event.get("content"),
                    "sig": event.get("sig"),
                    "user_count" :project_data["user_count"],
                    "event_count" : project_data["event_count"],
                    "records_count" : project_data["records_count"],
                    "event_type" : project_data["event_type"]
                }
            ]
        }
        print(mutation)
        insert_dgraph(mutation)
        print("Project event stored successfully.")

def on_error(ws, error):
    print("Error:", error)

def on_close(ws, close_status_code, close_msg):
    print("WebSocket closed")

def on_open(ws):
    print("Connected to Nostr Relay")
    subscription_id = str(uuid.uuid4()) 
    filters = {"kinds": [1], "limit": 10}   
    filters = {"kinds": [2323],}   

    # filters = {}   

    message = ["REQ", subscription_id, filters]
    ws.send(json.dumps(message))
    print(f"Sent subscription request: {message}")

def start_websocket():
    ws = websocket_client.WebSocketApp(
        RELAY_URL,
        on_message=on_message,
        on_error=on_error,
        on_close=on_close
    )
    ws.on_open = on_open
    ws.run_forever()

threading.Thread(target=start_websocket, daemon=True).start()

app = Flask(__name__)
CORS(app)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)