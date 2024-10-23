# Web workers as actors in Rust webapps

This repository is a template for experimenting with web workers as actors in a Rust webapp. The template demonstrates how to render the Mandelbrot set in a Rust webapp using a web worker. For detailed information about this template and alternatives, refer to my blog post: [Using Web Workers in Rust Webapps](https://allwright.io/#/blog/20241016-using-web-workers.md).

## Quick start

### GitHub Codespace
To play around with this repository and run the example without downloading or installing anything, click the green button labeled **Use this template** in the top-right corner of the GitHub user interface and select **Open in a codespace**. This will open the repository in Visual Studio Code for the Web and build the included Docker image. After the image is built, you can run the example by typing `python3 run.py` into VS Code's terminal. Note that each free GitHub account includes 120 core-hours for free.

### Local development
If you would like to play around with this repository locally, start by clicking the green button labeled **Use this template** in the top-right corner of the GitHub user interface and select **Create a new repository**. This will copy this repository to your account with a clean git history.

You can then clone either this repository or the new repository that you just created to your local machine. A `Dockerfile` which satisfies all the necessary dependencies is included in the `.devcontainer` directory. This `Dockerfile` can be built manually or using [VS Code's Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension (recommended).

Once the container is running, enter `python3 run.py` into VS Code's terminal to build the crates, run wasm-bindgen, and start the Python web server on `http://localhost:3000`.

## Repository structure

| File          | Description                                                          |
|---------------|----------------------------------------------------------------------|
| README.md     | This file                                                            |
| Cargo.toml    | Cargo workspace configuration                                        |
| actor         | Actor crate for the code that runs in the web worker                 |
| main          | Main crate for the code  that runs in the main thread                |
| shared        | Shared crate for common code used by both the actor and main crates |
| run.py        | Development script to build and serve the code                       |
| index.html    | Static HTML document to be loaded                                    |
| .cargo        | Cargo configuration directory                                        |
| .devcontainer | Contains Dockerfile and devcontainer.json for VSCode                 |
| .gitignore    | Rules for ignoring the output and target directories in git          |