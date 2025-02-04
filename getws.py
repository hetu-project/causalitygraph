import asyncio
import websockets

# 定义处理 WebSocket 连接的函数
async def handle_connection(websocket, path):
    print("客户端已连接")
    try:
        async for message in websocket:
            # 接收客户端发送的消息
            print(f"收到消息: {message}")
            # 可以在这里回复客户端
            await websocket.send(f"服务器已收到: {message}")
    except websockets.ConnectionClosed:
        print("客户端断开连接")
    except Exception as e:
        print(f"发生错误: {e}")

# 启动 WebSocket 服务器
async def start_server():
    server = await websockets.serve(handle_connection, "144.126.138.135", 10547)
    print(f"WebSocket 服务器已启动，正在监听 ws://144.126.138.135:10547")
    await server.wait_closed()  # 保持服务器运行

# 运行服务器
asyncio.run(start_server())