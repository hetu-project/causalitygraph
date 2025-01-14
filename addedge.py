import requests
import json


mutation = {
  "set": [
    {
      "uid": "0x2868",
      "author": {
        "uid": "0x2869"
      }
    }
  ]
}

url = "http://144.126.138.135:8080/mutate?commitNow=true"
headers = {"Content-Type": "application/json"}

try:
    response = requests.post(url, headers=headers, data=json.dumps(mutation))
    response.raise_for_status()
    print("边插入成功:", response.json())
except Exception as e:
    print(f"边插入失败: {e}")