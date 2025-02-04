import json
import time
import threading
import websockets
from websockets.server import serve

# 测试事件数据
test_events = [
    # [
    # "EVENT",
    # "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
    #     {
    #     "id": "b2f8d19545dadd2802e0a474927ae81f90f7228222f0c2dbc774ff07b182bbdd",
    #     "pubkey": "df57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d049124g", 
    #     "created_at": 1673819380,
    #     "kind": 2323,
    #     "tags": [
    #         ["i", "invite"],
    #         ["LamportID","1"],
    #         ["p","hetu round1"],
    #         ["invitee","2"],
    #     ],
    #     "content": "LamportID:alice123",
    #     "sig": "4e1452d4e0d2f2d72cbe6f87b041ee449526448df97a51c7e06f615043208d99ee41b04267df317b0c0597f5d5cc118160531f5edb97c34517a2ea3281db5f54"
    #     }
    # ],
            [
    "EVENT",
    "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
        {
        "id": "b2f8d19545dadd2802e0a474927ae81f90f7228222f0c2dbc774ff07b182bbed",
        "pubkey": "6126fcf2a10f60decf9b8c8d8617fbf67fb72d5926ef95963e7406e9c385cc77",
        "created_at": 1673819380,
        "kind": 30050,
        "tags": [
            ["project_name", "hetu1"],
            ["user_count", 0],
            ["event_count", 0],
            ["records_count", 0],
            ["event_type", "invite"],
            ["event_type", "sign_in"]
        ],
        "content": "This is a description of my awesome project.",
        "sig": "4e1452d4e0d2f2d72cbe6f87b041ee449526448df97a51c7e06f615043208d99ee41b04267df317b0c0597f5d5cc118160531f5edb97c34517a2ea3281db5f54"
        }
        ]
    #     [
    # "EVENT",
    # "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
    #     {
    #     "id": "b2f8d19545dadd2802e0a474927ae81f90f7228222f0c2dbc774ff07b182bbdd",
    #     "pubkey": "bec111374c2cf0a0ef3944b0faf0bc175b9ea66aba0e92730bb057f7192344a3",
    #     "created_at": 1673819380,
    #     "kind": 30050,
    #     "tags": [
    #         ["project_name", "hetu2"],
    #         ["user_count", 0],
    #         ["event_count", 0],
    #         ["records_count", 0],
    #         ["event_type", "invite"],
    #         ["event_type", "sign_in"]
    #     ],
    #     "content": "This is a description of my awesome project.",
    #     "sig": "4e1452d4e0d2f2d72cbe6f87b041ee449526448df97a51c7e06f615043208d99ee41b04267df317b0c0597f5d5cc118160531f5edb97c34517a2ea3281db5f54"
    #     }
    #     ]
    # [
    # "EVENT",
    # "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
    #     {
    #     "id": "b1f8d19545dadd2802e0a474927ae81f90f7228222f0c2dbc774ff07b182bbdd",
    #     "pubkey": "df57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d049124g", 
    #     "created_at": 1673819380,
    #     "kind": 2321,
    #     "tags": [
    #         ["LamportID", "1"],
    #         ["Tiwtter","Lance"]
    #     ],
    #     "content": "LamportID:alice123",
    #     "sig": "4e1452d4e0d2f2d72cbe6f87b041ee449526448df97a51c7e06f615043208d99ee41b04267df317b0c0597f5d5cc118160531f5edb97c34517a2ea3281db5f54"
    #     }
    # ],
    # [
    # "EVENT",
    # "840d1efe-9df7-423b-a3ba-d1ebac97abb1",
    #     {
    #     "id": "b1f8d19545dadd2802e0a474927ae81f90f7228222f0c2dbc774ff07b182bbdd",
    #     "pubkey": "df57b4986a2c659965c3df95ca3fea3533a207b09bf2c55a70d406c7d049124g", 
    #     "created_at": 1673819380,
    #     "kind": 2321,
    #     "tags": [
    #         ["LamportID", "1"],
    #         ["Tiwtter","Lance"]
    #     ],
    #     "content": "LamportID:alice123",
    #     "sig": "4e1452d4e0d2f2d72cbe6f87b041ee449526448df97a51c7e06f615043208d99ee41b04267df317b0c0597f5d5cc118160531f5edb97c34517a2ea3281db5f54"
    #     }
    # ]
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