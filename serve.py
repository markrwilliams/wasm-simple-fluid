import os
import http.server
import socketserver

PORT = 8080

Handler = http.server.SimpleHTTPRequestHandler

Handler.extensions_map[".wasm"] = "application/wasm"

class UsefulTCPServer(socketserver.TCPServer):
    allow_reuse_address = True

with UsefulTCPServer(("", PORT), Handler) as httpd:
    print("serving at port", PORT)
    os.chdir("www")
    httpd.serve_forever()
