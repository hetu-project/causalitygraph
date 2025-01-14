import json
import time
import threading
import websockets
from websockets.server import serve

# 测试事件数据
test_events = [
    [
    "EVENT",
    "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
    {
        "content": "2117",
        "created_at": 1736496186,
        "id": "cf159af242803f1eaea6fbd65f24481f62f1e526beb0efd17ba8c4f2923b02a2",
        "kind": 1,
        "pubkey": "ef57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d0491250",
        "sig": "8f51e15b023c83cbfe0676366c8f12c184c7727407dd5ef854ed250852f7cbade2ab48b2969dda83f61c70a78b8a77c7f9c7ab20eb718125b57ef3dd54e12321",
        "tags": [
            # ["e", "c2f8d21ac8be14592f06991a34dc8aa7835eb0c569ed1d084dd2c78cb84abf96", "wss://nostr.einundzwanzig.space", "mention"],
                ["t", "staySAIF"]
        ]
    }
    # {
    #     "content": "first",
    #     "created_at": 1736477404,
    #     "id": "b1f8d19545dadd2802e0a474927ae81f90f7228222f0c2dbc774ff07b182bbdd",
    #     "kind": 1,
    #     "pubkey": "df57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d049124f",
    #     "sig": "4e1452d4e0d2f2d72cbe6f87b041ee449526448df97a51c7e06f615043208d99ee41b04267df317b0c0597f5d5cc118160531f5edb97c34517a2ea3281db5f54",
    #     # "tags": [
    #     # ["p", "460c25e682fda7832b52d1f22d3d22b3176d972f60dcdc3212ed8c92ef85065c", "", "mention"]
    #     # ]
    # }
    ],
    [
        "EVENT",
        "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
        {
            "content": "2118",
            "created_at": 1736496191,
            "id": "b203958795e76ba9b934e75b12e75e5c4aff6c4c84c61a0c005d7a3987659342",
            "kind": 1,
            "pubkey": "bf57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d0491250",
            "sig": "6f0c01dc9d6ada2256a2c1f0e2c60c563d61d33b30fa133d4ed56972d7e038bfeabdad3ee1d7f95493872b2b592f69604282d84ca2aeb8f953245d8f4ef72506",
            # "tags": [
            #     # ["e", "c2f8d21ac8be14592f06991a34dc8aa7835eb0c569ed1d084dd2c78cb84abf96", "wss://nostr.einundzwanzig.space", "root"],
            #     ["p", "dd664d5e4016433a8cd69f005ae1480804351789b59de5af06276de65633d319"]
            # ]
            "tags": [
                ["t", "staySAIF"]
            ]
        }
        # {
        #     "content": "second.\n\nhttps://cdn.yandere.love/6e/6c/79/6e6c793fc7f4979e49a9a3ad62b22f3fdbc913812d478ee06d6456c63b7dccb6.jpg",
        #     "created_at": 1736477397,
        #     "id": "71445ad2a4759ae75ab8d4a767e8f6f8d5054b38871658ff7442425853780fc1",
        #     "kind": 1,
        #     "pubkey": "df57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d049124f",
        #     "sig": "8c4d44a34f2cf3bb96b0cef35882ba86f3ea6e0715415b3d3d35345faa7f785503d1d220f58127acde4a1edb44e2f7d9b733ab11919acda5810c0d1dfee4f656",
        #     # "tags": [
        #     # ["imeta", "url https://cdn.yandere.love/6e/6c/79/6e6c793fc7f4979e49a9a3ad62b22f3fdbc913812d478ee06d6456c63b7dccb6.jpg", "m image/jpeg"],
        #     # ["proxy", "https://misskey.yandere.love/objects/0ed36537-cbbb-41f7-be41-f20e9fe61da9", "activitypub"]
        #     # ]
        # }
        ],
        # [
        #     "EVENT",
        #     "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
        # {
        #     "content": "2119",
        #     "created_at": 1736496193,
        #     "id": "6f0702598ee8dc290876134fed82301d97507f1bc51cb2383cb8b7c7814613b8",
        #     "kind": 1,
        #     "pubkey": "df57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d0491248",
        #     "sig": "77539499be3129ada4e6cc8e2c9ffde78cee7bc0784d7b3c3b03588c936f78edf2cd1b6d647e4a2991bd307928848655fd0c17f0c1da4caf90a7126f841baf9c",
        #     "tags": [
        #         # ["e", "c2f8d21ac8be14592f06991a34dc8aa7835eb0c569ed1d084dd2c78cb84abf96", "wss://nostr.einundzwanzig.space", "root"],
        #         ["p", "dd664d5e4016433a8cd69f005ae1480804351789b59de5af06276de65633d319"]
        #     ]
        #     }]
]

# WebSocket 服务逻辑
async def nostr_relay(websocket):
    print("Client connected")
    for event in test_events:
        await websocket.send(json.dumps(event))
        print(f"Sent event: {event}")
        time.sleep(2)  # 每隔 2 秒发送一个事件

# 启动 WebSocket 服务
async def start_websocket_server():
    async with serve(nostr_relay, "localhost", 8765):
        print("WebSocket server started on ws://localhost:8765")
        await asyncio.Future()  # 保持服务运行

# 运行 WebSocket 服务
if __name__ == "__main__":
    import asyncio
    asyncio.run(start_websocket_server())