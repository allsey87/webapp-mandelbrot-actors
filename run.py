# This is a quick script for demonstrating a webapp using actors, it will build everything needed
# and create an output directory, containing the assets that need to be served. The script will
# then start a Python HTTP server. This is absolutely not meant to be used for production. At a
# bare minimum, you would want to build in release mode, optimise the WebAssembly module using
# wasm-opt, and use something more robust than Python's HTTP server.
#
# If you need support with your webapp, get in touch via contact@allwright.io and tell me about
# your project.

import http.server
import os
import signal
import socketserver
import subprocess

PORT = 3000

# build project
subprocess.run([
    "cargo",
    "build"
])

# create the output directory and symlink index.html
if not os.path.isdir('output'):
    os.mkdir('output')
if not os.path.islink('output/index.html'):
    os.symlink('../index.html', 'output/index.html')

# generate bindings
subprocess.run([
    "wasm-bindgen",
    "target/wasm32-unknown-unknown/debug/main.wasm",
    "--out-dir",
    "output",
    "--target",
    "web",
    "--no-typescript"
])
subprocess.run([
    "wasm-bindgen",
    "target/wasm32-unknown-unknown/debug/actor.wasm",
    "--out-dir",
    "output",
    "--target",
    "web",
    "--no-typescript"
])

# patch actor.js to start automatically in the worker (not needed when using a bundler)
with open('output/actor.js', 'r') as actor_js:
    actor_js_content = actor_js.read()
actor_js_content = \
    actor_js_content.replace('export default __wbg_init;', '__wbg_init();')
with open('output/actor.js', 'w') as actor_js:
    actor_js.write(actor_js_content)

# start web server
class RequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory='output', **kwargs)
server = socketserver.TCPServer(('localhost', PORT), RequestHandler)
signal.signal(signal.SIGINT,
    lambda _signal, _frame: setattr(server, '_BaseServer__shutdown_request', True))
print('serving on http://localhost:{}/'.format(PORT))
server.serve_forever()
