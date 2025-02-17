import json
import threading
from flask import Flask, request, jsonify
from ariadne import QueryType, make_executable_schema, graphql_sync
import pydgraph
import websocket as websocket_client
from ariadne import QueryType, make_executable_schema
import requests
from ariadne.wsgi import GraphQL
from flask_cors import CORS

DGRAPH_URI = "144.126.138.135:9080" 


headers = {"Content-Type": "application/json"}


def search_users(search_value = None):
    # exit()
    query_template = """
    {
        user(func: %s) {
            uid
            dgraph.type
            content
            name
            lamport_id
            created_at
            pubkey
            twitter_id
            eth_address
            posts {
                uid
                content
            }
            invite @facets {
                uid
                lamport_id
                facets {
                    project_name
                    content
                    created_at
                }
            }
            ~invite @facets {
                uid
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
            }
            create_votes {
                uid
                lamport_id
            }
        }
    }
    """
    
    if search_value:
        func_condition = f'or(eq(lamport_id, {search_value}), eq(twitter_id, {search_value}), eq(eth_address, {search_value}))'
        func_condition = f'eq(eth_address, {search_value})'
        func_condition = f'uid({search_value})'
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
        # print(result)
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
                event =set()
                user_detail = {}
                user_detail = {'pubkey':pubkey, 'content':content, 'created_at':created_at, 'lamport_id':lamport_id, 'twitter_id':twitter_id, 'eth_address':eth_address,}

                for predicate, value in node.items():
                    if predicate in ['posts', 'invite', 'create_votes']:
                        user_detail[predicate] = node[predicate]
                        event.add(predicate)
                        for one_value in value:
                            one_edge = {"uid": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'], 'category':predicate}
                            edges.append(one_edge)
                
                user_detail['event_type'] =list(event)
                one_user = {'uid':uid, 'category':node_type, 'label':lamport_id, 'detail':user_detail}
                users.append(one_user)
            # print(edges)
            # print(users)
            return users, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("no user")
    # except Exception as e:
    #     print(f"error: {e}")


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
                print(f"pot  UID: {uid}, type: {node_type}")
                event =set()
                project_detail = {'event_type':event_type,  'event_count':event_count, 'records_count':records_count}

                for predicate, value in node.items():
                    if predicate in ['~participates_in']:
                        event.add(predicate)
                        if predicate == '~participates_in':
                            project_detail['members'] = node[predicate]
                        for one_value in value:
                            print(one_value)
                            print(f"edge: {uid} -> {one_value['uid']}")
                            one_edge = {"uid": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'category':'members'}
                            edges.append(one_edge)
                            user_count += 1
                    elif predicate in ['created_by']:
                        print(value)
                        print(f"edge: {uid} -> {value['uid']}")
                        one_edge = {"uid": f"{uid}_{value['uid']}", 'source':uid, 'target':value['uid'], 'category':predicate}
                        edges.append(one_edge)
                        user_count += 1
                
                project_detail['user_count'] = user_count
                one_project = {'uid':uid, 'category':node_type, 'label':project_name, 'created_at':created_at, 'content':content, 'detail':project_detail}
                projects.append(one_project)

            print(edges)
            print(projects)
            return projects, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("no projects")
    # except Exception as e:
    #     print(f"error: {e}")


def search_posts():
    # exit()
    query = {
        "query": """
        {
            post(func: type(Post)) {   
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

    url = "http://144.126.138.135:8080/query"
    url = "http://212.56.40.235:8080/query"

    headers = {"Content-Type": "application/json"}

    try:
        response = requests.post(url, headers=headers, data=json.dumps(query))
        response.raise_for_status()    
        result = response.json()
        print(result)
           
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
                print(f"port UID: {uid}, type: {node_type}")
                for predicate, value in node.items():
                    if predicate in ['reply', 'root','tags','mention_p']:
                        if type(value) == list:
                            for one_value in value:
                                print(one_value, value)
                                print(f"edge: {uid} -> {one_value['uid']}")
                                one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':'post'}
                                edges.append(one_edge)
                        else:
                            one_value = value
                            print(f"edge: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':'post'}
                            edges.append(one_edge)                            
            print(posts)
            return posts, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("no posts")
    except Exception as e:
        print(f"error: {e}")


def search_tags():
    # exit()
    query = {
        "query": """
        {
            tag(func: type(Tag)) {   
                uid
                dgraph.type
                posts{
                    uid
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
                print(f"port UID: {uid}, type: {node_type}")
                for predicate, value in node.items():
                    if predicate == 'posts':
                        for one_value in value:
                            print(one_value, value)
                            print(f"edge: {uid} -> {one_value['uid']}")
                            one_edge = {"id": f"{uid}_{one_value['uid']}", 'source':uid, 'target':one_value['uid'],'label':one_value['uid'], 'category':'post'}
                            edges.append(one_edge)
                      
            print(edges)
            # print(tags)
            return tags, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("no tags")
    except Exception as e:
        print(f"error: {e}")


def search_votes(search_value = None):
    query_template = """
    {
        vote(func: %s) {
            uid
            id
            dgraph.type
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
    
    if search_value:
        func_condition = f'or(eq(vote_title, {search_value}), eq(twitter_id, {search_value}), eq(eth_address, {search_value}))'
        func_condition = f'eq(vote_title, {search_value})'

    else:
        func_condition = 'type(Vote)'
    
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
        votes = []
        nodes = result.get("data", {}).get("vote", [])
        if nodes:
            for node in nodes:
                uid = node.get("uid")
                node_type = node.get("dgraph.type")[0]
                created_at = node.get('created_at')
                content = node.get('content')
                vote_title = node.get('vote_title')
                vote_options = node.get('vote_options')
                vote_detail = {'vote_title':vote_title, 'content':content, 'created_at':created_at, 'vote_options':vote_options, }
                one_user = {'uid':uid, 'category':node_type, 'label':vote_title, 'detail':vote_detail}
                votes.append(one_user)
            # print(edges)
            # print(users)
            return votes, edges
            # print(json.dumps(nodes, indent=2, ensure_ascii=False))
        else:
            print("no votes")
    # except Exception as e:
    #     print(f"error: {e}")


app = Flask(__name__)
CORS(app)


@app.route('/all_posts')
def all_posts():
    posts, edges = search_posts()
    response = {
        'nodes': posts,
        'edges': edges
    }
    return jsonify(response)       

@app.route('/all_users')
def all_users():
    users, edges = search_users()
    response = {
        'nodes': users,
        'edges': edges
    }
    return jsonify(response)       

@app.route('/all_projects')
def all_projects():
    projects, edges = search_projects()
    response = {
        'nodes': projects,
        'edges': edges
    }
    return jsonify(response)       

@app.route('/user/<search_value>')
def get_user(search_value):
    user, edges = search_users(search_value)
    if user:
        response = {
            'node': user,
            'edges': edges
        }
        return jsonify(response)       
    else:
        return jsonify({'error': 'User not found'}), 404  

@app.route('/all_tags')
def all_tags():
    tags, edges = search_tags()
    response = {
        'nodes': tags,
        'edges': edges
    }
    return jsonify(response)       

@app.route('/all_votes')
def all_votes():
    votes, edges = search_votes()
    response = {
        'nodes': votes,
        'edges': edges
    }
    return jsonify(response)       

@app.route('/ttt')
def ttt():
    return f'sss'

@app.route('/all_data')
def all_data():
    all_nodes = []
    all_edges = []

    # posts, edges = search_posts()
    # all_nodes.extend(posts)
    # all_edges.extend(edges)


    votes, edges = search_votes()
    all_nodes.extend(votes)
    all_edges.extend(edges)

    projects, edges = search_projects()
    all_nodes.extend(projects)
    all_edges.extend(edges)
    
          
    users, edges = search_users()
    all_nodes.extend(users)
    all_edges.extend(edges)

    # tags, edges = search_tags()
    # all_nodes.extend(tags)
    # all_edges.extend(edges)

    response = {
        'nodes': all_nodes,
        'edges': all_edges
    }
    return jsonify(response)       

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


@app.route("/graphql", methods=["GET", "POST"])
def graphql_server():
    if request.method == "GET":
        return graphql_app.handle_request(request.environ, request.start_response)
    else:
        data = request.get_json()
        success, result = graphql_app.execute_query(request.environ, data)
        status_code = 200 if success else 400
        return jsonify(result), status_code

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5006, debug=True)




