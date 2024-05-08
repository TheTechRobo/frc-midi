import asyncio

BUTTONS = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "MOD"
]

async def _server_callback(reader: asyncio.StreamReader, writer: asyncio.StreamWriter):
    print("Received connection")
    writer.write(b"GO!")
    await writer.drain()
    print("Awaiting data")
    while msgb := await reader.read(1):
        msg = msgb[0]
        type = msg >> 6
        msg &= 0b00111111
        if type == 1:
            print(f"Button Press: {BUTTONS[msg]}")
        elif type == 2:
            if msg & 1:
                print("Dial Movement: Left")
            else:
                print("Dial Movement: Right")
        elif type == 3:
            print(f"Button Release: {BUTTONS[msg]}")
        else:
            print("Unknown Message", msgb)
        await asyncio.sleep(0)

async def server_callback(reader: asyncio.StreamReader, writer: asyncio.StreamWriter):
    try:
        return await _server_callback(reader, writer)
    except Exception as e:
        print(f"client disconnected ({repr(e)})")

async def w():
    print("Listening on 127.0.0.1:7757")
    server = await asyncio.start_server(server_callback, "127.0.0.1", 7757)
    async with server:
        await server.serve_forever()

asyncio.run(w())
