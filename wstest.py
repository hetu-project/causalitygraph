import json
import time
import threading
import websockets
from websockets.server import serve

# 测试事件数据
test_events = [
['EVENT', 'd892cae1-ec57-4087-86b5-ea5e3937b25c', {'kind': 2410, 'id': 'de5d7b9f699096a320bf1351ae76d346e8caa9f4a212367bcdb6e02238c11b26', 'pubkey': '79dff8f82963424e0bb02708a22e44b4980893e3a4be0fa3cb60a43b946764e3', 'created_at': 1738277694, 'tags': [['LamportID', '1'], ['vote_id', 'a6d73ab3-ba70-4490-88d6-8c645a49fc19'], ['title', 'test3'], ['content', '6 vote for test3'], ['start_time', '2025-01-31 10:31:58.293877 +00:00'], ['end_time', '2025-02-03 08:00:00 +00:00'], ['options', 'For,Against,Abstain'], ['sig', '']], 'content': '6 Vote a6d73ab3-ba70-4490-88d6-8c645a49fc19, Title:test3', 'sig': '879eb383276313493f249a14c30c029a3fda84c8b29423a8ac6b06bd2fcc76505fa6b2781f23fdc56eac64c131f5053650f0469abd1b71ca0b8c56c8de9246c1', 'uday_nonce': 46306}],
['EVENT', 'd892cae1-ec57-4087-86b5-ea5e3937b25c', {'kind': 2411, 'id': '201779ecde72a46db6c38e8d8ec4f26812ec767a868e6904c158f23a6b6e1a93', 'pubkey': '79dff8f82963424e0bb02708a22e44b4980893e3a4be0fa3cb60a43b946764e3', 'created_at': 1738377691, 'tags': [['LamportID', '1'], ['vote_id', 'a6d73ab3-ba70-4490-88d6-8c645a49fc19'], ['title', 'test2'], ['content', '6 vote for test2'], ['start_time', '2025-01-31 13:24:05.128740 +00:00'], ['end_time', '2025-02-04 08:00:00 +00:00'], ['options', 'For,Against,Abstain'], ["selection", "For"], ['sig', '']], 'content': '6 Vote cb91a1a4-5624-408c-819f-f8597c0ebbf1, Title:test2', 'sig': 'c1330a82934cd082333554e60ead0798e42724aa77cc3af0cf2312a5761c258d161800bdafb67d6c8085b6acf78bbe7bc76552e934b9cf09152b103a07b13e42', 'uday_nonce': 46305}],
['EVENT', 'd892cae1-ec57-4087-86b5-ea5e3937b25c', {'kind': 2411, 'id': 'd6fa29a4917dfc406f8bec1ddb523b6cc190ca8b55d4b60b5215cf92f508b66f', 'pubkey': '79dff8f82963424e0bb02708a22e44b4980893e3a4be0fa3cb60a43b946764e3', 'created_at': 1738377635, 'tags': [['LamportID', '3'], ['vote_id', 'a6d73ab3-ba70-4490-88d6-8c645a49fc19'], ['title', 'test3'], ['content', '6 vote for test3'], ['start_time', '2025-01-31 13:46:51.408563 +00:00'], ['end_time', '2025-02-03 13:46:39 +00:00'], ['options', 'For,Against,Abstain'], ["selection", "Against"], ['sig', '']], 'content': '6 Vote 8d76a727-4a8b-4350-9a13-e02482a84623, Title:test3', 'sig': '49a9faa60f7c1168f128a34d6981e6e2b442ed3af3531de2fc0723c64134a20c732a61c23a2bfe11177ccf7db5c2974ae16d391c6f556456daf8133ed33bf496', 'uday_nonce': 46304}],
['EVENT', 'd892cae1-ec57-4087-86b5-ea5e3937b25c', {'kind': 2411, 'id': '5fb7c7d45becc661cce6bee396586e911f8c8dd16f0c29df83daed905a1236a8', 'pubkey': '79dff8f82963424e0bb02708a22e44b4980893e3a4be0fa3cb60a43b946764e3', 'created_at': 1738329887, 'tags': [['LamportID', '4'], ['vote_id', 'a6d73ab3-ba70-4490-88d6-8c645a49fc19'], ['title', 'test1'], ['content', '1 vote for test1'], ['start_time', '2025-01-31 13:24:32.982145 +00:00'], ['end_time', '2025-02-04 08:00:00 +00:00'], ['options', 'For,Against,Abstain'], ["selection", "Against"], ['sig', '']], 'content': '1 Vote 4206cc82-990d-40ec-b75f-43647d1b7564, Title:test1', 'sig': '3d530aee3f82adaa1e4289c7525eb9557250bde73f15cf94baad0796ab710173b7c72274b0981596e9e8eede026a7515281ca130831c4c38a450f3bb034058bb', 'uday_nonce': 46302}],
['EVENT', 'd892cae1-ec57-4087-86b5-ea5e3937b25c', {'kind': 2411, 'id': 'd993cc394c6bbaa86f5e308b2f8239c0f9882168a7539159a61292ed7ccea435', 'pubkey': '79dff8f82963424e0bb02708a22e44b4980893e3a4be0fa3cb60a43b946764e3', 'created_at': 1738329861, 'tags': [['LamportID', '5'], ['vote_id', 'a6d73ab3-ba70-4490-88d6-8c645a49fc19'], ['title', 'test2'], ['content', '1 vote for test2'], ['start_time', '2025-01-31 13:24:05.128740 +00:00'], ['end_time', '2025-02-04 08:00:00 +00:00'], ['options', 'For,Against,Abstain'], ["selection", "Abstain"], ['sig', '']], 'content': '1 Vote 40d1e7ac-f631-42c0-9c4b-8abe06501311, Title:test2', 'sig': 'edac8c530ac6550d1629351dbc14d67f19e9b21b83e388a41c83a87fed871e341172648cc73ded2193c74f4d8bdee0364a45caa3cf44b34fdce281601bf396e3', 'uday_nonce': 46301}]

]

test_events = [
['EVENT', '0773887e-0061-4cb1-8772-2e375c88f7ad', {'kind': 2323, 'id': '7ee1e233011c573b0a3974313694857f45336870fc0827c3a890d03ebc0ccfd8', 'pubkey': 'b3ed835ef25d8b3843ec991fc51cda06d09ecee22f8147865050323049d560ae', 'created_at': 1737982742, 'tags': [['i', 'invite'], ['LamportId', '150'], ['invitee', '9827888'], ['p', 'hetu2'], ['lmport_type', 'invite']], 'content': '150 邀请 9827', 'sig': 'aeef53c6144eea9a60e2492a1edaab7dfceedf9f7fba26b838635f0b22ac815d88f7c63e2354e3b4e57946d6944667c88d3a3d00486d9d795b6a9875135dc2fe', 'uday_nonce': 43182}]
]

test_events = [
    ['EVENT','0773887e-0061-4cb1-8772-2e375c88f7ad',    
     {
        "id": "b7f69c3dffacb073233cc293b2e24c02cfef1761a1e5e98b054ec456d349c71f",
        "pubkey": "abcd1234ef567890abcd1234ef567890abcd1234ef567890abcd1234ef567890",
        "created_at": 1707075265,
        "kind": 3,
        "tags": [
            ["p", "ef567890abcd1234ef567890abcd1234ef567890abcd1234ef567890abcd1234"],
            ["p", "1234abcd5678ef901234abcd5678ef901234abcd5678ef901234abcd5678ef90"],
            ["p", "7890ef1234abcd567890ef1234abcd567890ef1234abcd567890ef1234abcd56"]
        ],
        "sig": "d01234abcd5678ef901234abcd5678ef901234abcd5678ef901234abcd5678ef901234abcd5678ef901234abcd5678ef901234abcd5678ef90"
    }
    ]
]



test_events = [
    ['EVENT','0773887e-0061-4cb1-8772-2e375c88f7ad',    
        {
            "id": "a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890",
            "pubkey": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            "created_at": 1707075265,
            "kind": 6,
            "tags": [
                ["t", "twitter"],
                ["account", "elonmusk"],
                ["user_id", "987654321"],
                ["username", "crypto_fan"],
                ["created_at", "1707075260"],
                ["post_id", "1234567890abcdef"]
            ],
            "content": "",
            "sig": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
        }
    ]
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