from flask import Flask, send_from_directory

app = Flask(__name__)


@app.route("/")
def index(name=None):
    return send_from_directory("", "index.html")


@app.route("/wasm/physics_engine.js")
def source(name=None):
    return send_from_directory("wasm", "physics_engine.js")


@app.route("/wasm/physics_engine_bg.wasm")
def wasm(name=None):
    return send_from_directory("wasm", "physics_engine_bg.wasm")


@app.route("/assets/happy-tree.png")
def happy_tree(name=None):
    return send_from_directory("assets", "happy-tree.png")
