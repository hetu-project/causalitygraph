import json
import threading
from flask import Flask, request, jsonify
from ariadne import QueryType, make_executable_schema, graphql_sync
import pydgraph
import websocket as websocket_client
import requests

# Dgraph 连接配置
DGRAPH_URI = "144.126.138.135:9080"  # Dgraph Alpha 的 gRPC 


headers = {"Content-Type": "application/json"}





def search_users():
    # exit()
    query = {
        "query": """
        {
            user(func: type(User)) {  # 替换为实际的帖子 UID
                uid
                dgraph.type
                id
                content
                name
                pubkey
                posts{
                    uid
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
                print(f"节点 UID: {uid}, 类型: {node_type}")
                for predicate, value in node.items():
                    if predicate == 'posts':
                        for one_value in value:
                            print(f"边: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'type':'post'}
                            edges.append(one_edge)
            print("查询成功，返回的帖子及其提到的用户数据:")
            print(edges)
            print(users)
            return users, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("查询成功，但未找到帖子数据。")
    except Exception as e:
        print(f"查询失败: {e}")



def search_posts():
    # exit()
    query = {
        "query": """
        {
            post(func: type(Post)) {  # 替换为实际的帖子 UID
                uid
                dgraph.type
                id
                content
                name
                pubkey
                created_at
                reply{
                    uid
                }
                root{
                    uid
                }
                tags{
                    uid
                }
                mention_p{
                    uid
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
        edges = []
        posts = []
        nodes = result.get("data", {}).get("post", [])
        if nodes:
            for node in nodes:
                uid = node.get("uid")
                node_type = node.get("dgraph.type")[0]
                pubkey = node.get("pubkey")
                created_at = node.get('created_at')
                content = node.get('content')

                one_post = {'id':uid, 'type':node_type, 'pubkey':pubkey, 'created_at':created_at, 'content':content }
                posts.append(one_post)
                print(f"节点 UID: {uid}, 类型: {node_type}")
                for predicate, value in node.items():
                    if predicate in ['reply', 'root','tags','mention_p']:
                        if type(value) == list:
                            for one_value in value:
                                print(one_value, value)
                                print(f"边: {uid} -> {one_value['uid']}")
                                one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'type':'post'}
                                edges.append(one_edge)
                        else:
                            one_value = value
                            print(f"边: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'type':'post'}
                            edges.append(one_edge)                            
            print("查询成功，返回的帖子及其提到的用户数据:")
            # print(edges)
            print(posts)
            return posts, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("查询成功，但未找到帖子数据。")
    except Exception as e:
        print(f"查询失败: {e}")



def search_tags():
    # exit()
    query = {
        "query": """
        {
            tag(func: type(Tag)) {  # 替换为实际的帖子 UID
                uid
                dgraph.type
                posts{
                    uid
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
        edges = []
        tags = []
        nodes = result.get("data", {}).get("tag", [])
        if nodes:
            for node in nodes:
                uid = node.get("uid")
                node_type = node.get("dgraph.type")[0]
                pubkey = node.get("pubkey")
                created_at = node.get('created_at')
                content = node.get('tag_content')

                one_post = {'id':uid, 'type':node_type, 'pubkey':pubkey, 'created_at':created_at, 'content':content }
                tags.append(one_post)
                print(f"节点 UID: {uid}, 类型: {node_type}")
                for predicate, value in node.items():
                    if predicate == 'posts':
                        for one_value in value:
                            print(one_value, value)
                            print(f"边: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'type':'post'}
                            edges.append(one_edge)
                      
            print("查询成功，返回的帖子及其提到的用户数据:")
            print(edges)
            # print(tags)
            return tags, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("查询成功，但未找到帖子数据。")
    except Exception as e:
        print(f"查询失败: {e}")


# 创建 Flask 应用并集成 GraphQL
app = Flask(__name__)



# 路由：获取所有帖子
@app.route('/all_posts')
def all_posts():
    posts, edges = search_posts()
    response = {
        'nodes': posts,
        'edges': edges
    }
    return jsonify(response)  # 返回 JSON 响应

# 路由：获取所有用户
@app.route('/all_users')
def all_users():
    users, edges = search_users()
    response = {
        'nodes': users,
        'edges': edges
    }
    return jsonify(response)  # 返回 JSON 响应

# 路由：获取所有标签
@app.route('/all_tags')
def all_tags():
    tags, edges = search_tags()
    response = {
        'nodes': tags,
        'edges': edges
    }
    return jsonify(response)  # 返回 JSON 响应

# 路由：获取所有数据
@app.route('/all_data')
def all_data():
    all_nodes = []
    all_edges = []

    # 获取帖子数据
    posts, edges = search_posts()
    all_nodes.extend(posts)
    all_edges.extend(edges)

    # 获取用户数据
    users, edges = search_users()
    all_nodes.extend(users)
    all_edges.extend(edges)

    # 获取标签数据
    tags, edges = search_tags()
    all_nodes.extend(tags)
    all_edges.extend(edges)

    response = {
        'nodes': all_nodes,
        'edges': all_edges
    }
    return jsonify(response)  # 返回 JSON 响应

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5005)




