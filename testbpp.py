import requests

query = """
{
    users {
        uid
    }
}
"""


url = "http://localhost:5005/graphql"
headers = {"Content-Type": "application/json"}

response = requests.post(url, headers=headers, json={"query": query})

if response.status_code == 200:
    print(response.json())
else:
    print(response.text)