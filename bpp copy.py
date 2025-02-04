import json
import threading
from flask import Flask, request, jsonify
from ariadne import QueryType, make_executable_schema, graphql_sync
import pydgraph
import websocket as websocket_client
from ariadne import QueryType, make_executable_schema
import requests
from ariadne.wsgi import GraphQL

DGRAPH_URI = "144.126.138.135:9080"  # Dgraph Alpha 的 gRPC 


headers = {"Content-Type": "application/json"}


def search_users(search_value = None):
    # exit()
    query_template = """
    {
        user(func: %s) {
            uid
            dgraph.type
            id
            content
            name
            lamport_id
            created_at
            pubkey
            twitter_id
            eth_address
            posts {
                uid
                id
                content
            }
            invite @facets {
                uid
                dgraph.type
                pubkey
                lamport_id
                facets {
                    project_name
                    content
                    created_at
                }
            }
            ~invite @facets {
                uid
                dgraph.type
                pubkey
                lamport_id
                facets: {
                    project_name
                    content
                    created_at
                }
            }
            participates_in {
                uid
                project_name
                dgraph.type
            }
        }
    }
    """
    
    if search_value:
        func_condition = f'or(eq(lamport_id, {search_value}), eq(twitter_id, {search_value}), eq(eth_address, {search_value}))'
        func_condition = f'eq(lamport_id, {search_value})'

    else:
        func_condition = 'type(User)'
    
    query = query_template % func_condition

    print(query)
    query = {"query": query}

    url = "http://144.126.138.135:8080/query"
    url = "http://212.56.40.235:8080/query"

    headers = {"Content-Type": "application/json"}
    print(query)
    # try:
    if True:
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
                lamport_id = node.get('lamport_id')
                twitter_id = node.get('twitter_id')
                eth_address = node.get('eth_address')
                projects = []
                print(f"节点 UID: {uid}, 类型: {node_type}")
                event =set()
                for predicate, value in node.items():
                    if predicate in ['posts', 'invite', 'participates_in']:
                        event.add(predicate)
                        for one_value in value:
                            print(one_value)
                            print(f"边: {uid} -> {one_value['uid']} , one_value\n{one_value}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':predicate}
                            edges.append(one_edge)
                            if 'project_name' in one_value:
                                one_edge['project_name'] = one_value['project_name']
                            if 'facets' in one_value and 'project_name' in one_value['facets']:
                                one_edge['project_name'] = one_value['facets']['project_name']
                            if predicate =='participates_in':
                                projects.append(one_value['project_name'])
                
                user_detail = {'pubkey':pubkey, 'content':content, 'created_at':created_at, 'lamport_id':lamport_id, 'twitter_id':twitter_id, 'eth_address':eth_address, 'event_type':list(event), 'projects':projects }
                one_user = {'id':uid, 'category':node_type, 'label':lamport_id, 'detail':user_detail}
                users.append(one_user)
            print("查询成功，返回的帖子及其提到的用户数据:")
            print(edges)
            print(users)
            return users, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("查询成功，但未找到帖子数据。")
    # except Exception as e:
    #     print(f"查询失败: {e}")


def search_projects(search_value = None):
    # exit()
    query_template = """
    {
        project(func: %s) {
            uid
            dgraph.type
            id
            content
            project_name
            created_at
            created_by {
                uid
                lamport_id
                pubkey
                content
            }
            ~participates_in {
                uid
                lamport_id
                pubkey
                content
            }
            user_count
            event_count
            records_count
            event_type
        }
    }
    """
    
    if search_value:
        func_condition = f'eq(project_name, {search_value})'
    else:
        func_condition = 'type(Project)'
    
    query = query_template % func_condition

    print(query)
    query = {"query": query}

    url = "http://144.126.138.135:8080/query"
    url = "http://212.56.40.235:8080/query"

    headers = {"Content-Type": "application/json"}
    print(query)
    # try:
    if True:
        response = requests.post(url, headers=headers, data=json.dumps(query))
        response.raise_for_status() 
        result = response.json()
        print(result)
        edges = []
        projects = []
        nodes = result.get("data", {}).get("project", [])
        if nodes:
            for node in nodes:
                uid = node.get("uid")
                node_type = node.get("dgraph.type")[0]
                project_name = node.get("project_name")
                created_at = node.get('created_at')
                content = node.get('content')
                user_count = node.get('user_count')
                event_type = node.get('event_type')
                event_count = node.get('event_count')
                records_count = node.get('records_count')
                print(f"节点 UID: {uid}, 类型: {node_type}")
                event =set()
                for predicate, value in node.items():
                    if predicate in ['~participates_in']:
                        event.add(predicate)
                        for one_value in value:
                            print(one_value)
                            print(f"边: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':'member'}
                            edges.append(one_edge)
                            user_count += 1
                    elif predicate in ['created_by']:
                        print(value)
                        print(f"边: {uid} -> {value['uid']}")
                        one_edge = {"id": f"{uid}_{value['uid']}", 'source':uid, 'target':value['uid'],'label':value['uid'], 'category':predicate}
                        edges.append(one_edge)
                        user_count += 1
                
                project_detail = {'user_count':user_count, 'event_type':event_type,  'event_count':event_count, 'records_count':records_count}
                one_project = {'id':uid, 'category':node_type, 'label':project_name, 'created_at':created_at, 'content':content, 'detail':project_detail}
                projects.append(one_project)
            print("查询成功，返回的帖子及其提到的用户数据:")
            print(edges)
            print(projects)
            return projects, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("查询成功，但未找到帖子数据。")
    # except Exception as e:
    #     print(f"查询失败: {e}")


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

                one_post = {'id':uid, 'label':uid, 'category':node_type, 'pubkey':pubkey, 'created_at':created_at, 'content':content }
                posts.append(one_post)
                print(f"节点 UID: {uid}, 类型: {node_type}")
                for predicate, value in node.items():
                    if predicate in ['reply', 'root','tags','mention_p']:
                        if type(value) == list:
                            for one_value in value:
                                print(one_value, value)
                                print(f"边: {uid} -> {one_value['uid']}")
                                one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':'post'}
                                edges.append(one_edge)
                        else:
                            one_value = value
                            print(f"边: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':'post'}
                            edges.append(one_edge)                            
            print("查询成功，返回的帖子及其提到的用户数据:")
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

                one_post = {'id':uid, 'category':node_type, 'label': uid, 'pubkey':pubkey, 'created_at':created_at, 'content':content }
                tags.append(one_post)
                print(f"节点 UID: {uid}, 类型: {node_type}")
                for predicate, value in node.items():
                    if predicate == 'posts':
                        for one_value in value:
                            print(one_value, value)
                            print(f"边: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':'post'}
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

@app.route('/all_projects')
def all_projects():
    projects, edges = search_projects()
    response = {
        'nodes': projects,
        'edges': edges
    }
    return jsonify(response)  # 返回 JSON 响应

@app.route('/user/<search_value>')
def get_user(search_value):
    user, edges = search_users(search_value)  # 假设你有一个函数 search_user_by_id 来根据 user_id 查询用户
    if user:
        response = {
            'node': user,
            'edges': edges
        }
        return jsonify(response)  # 返回 JSON 响应
    else:
        return jsonify({'error': 'User not found'}), 404  # 如果用户不存在，返回 404 错误

# 路由：获取所有标签
@app.route('/all_tags')
def all_tags():
    tags, edges = search_tags()
    response = {
        'nodes': tags,
        'edges': edges
    }
    return jsonify(response)  # 返回 JSON 响应


@app.route('/ttt')
def ttt():
    return f'sss'

# 路由：获取所有数据
@app.route('/all_data')
def all_data():
    all_nodes = []
    all_edges = []

    # 获取帖子数据
    # posts, edges = search_posts()
    # all_nodes.extend(posts)
    # all_edges.extend(edges)


    projects, edges = search_projects()
    all_nodes.extend(projects)
    all_edges.extend(edges)
    
    # 获取用户数据
    users, edges = search_users()
    all_nodes.extend(users)
    all_edges.extend(edges)

    # 获取标签数据
    # tags, edges = search_tags()
    # all_nodes.extend(tags)
    # all_edges.extend(edges)

    response = {
        'nodes': all_nodes,
        'edges': all_edges
    }
    return jsonify(response)  # 返回 JSON 响应

# 定义 GraphQL schema
type_defs = """
    type User {
        id: ID!
        category: String!
        label: String!
    }

    type Post {
        id: ID!
        label: String!
        category: String!
        pubkey: String
        created_at: String
        content: String
    }

    type Tag {
        id: ID!
        category: String!
        label: String!
        pubkey: String
        created_at: String
        content: String
    }

    type Query {
        users(search_value: String): [User]
        posts: [Post]
        tags: [Tag]
    }
"""
query = QueryType()
schema = make_executable_schema(type_defs, query)
graphql_app = GraphQL(schema, debug=True)

# 添加 GraphQL 路由
@app.route("/graphql", methods=["GET", "POST"])
def graphql_server():
    if request.method == "GET":
        # 返回 GraphQL Playground
        return graphql_app.handle_request(request.environ, request.start_response)
    else:
        # 处理 GraphQL 查询
        data = request.get_json()
        success, result = graphql_app.execute_query(request.environ, data)
        status_code = 200 if success else 400
        return jsonify(result), status_code

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5005, debug=True)




