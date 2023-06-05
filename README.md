# Online Clipboard

Modeled after the [Rust Warp Websocket server example](https://tms-dev-blog.com/build-basic-rust-websocket-server/) on TMS Developer Blog and the [tutorial github project](https://github.com/tmsdev82/basic-warp-websocket-server-tutorial), I decided to make it into a bundled application with hosting configurations and a frontend that could make use of the data.

There has since been more comprehensive examples in [their subsequent posts](https://tms-dev-blog.com/warp-data-update-loop-easy-how-to/) but I am just setting it up as a proof con concept with as little dependency as possible and keeping it simple.

## What does it do

This project is a simplistic singleton clipboard that updates real time online across multiple clients. This is more so an exercise in memory-safe and high performance json parsing and working with the borrow checker in Rust. There are many quality of life improvements I can perform but this is just a short exercise.

Feel free to visit the demo on https://harrytsangprivate.csproject.org/clipboard/ and leave a message. I don't even log the messages

## Important Folders
- backend: A Rust websocket backend on ws://localhost:8000/ws and static http server on http://localhost:8000/clipboard
- frontend: A React project with a simple text field that calls to the reflects from the websocket servet

## Building

The Dockerfile and compose examples are provided to build a minimalistic docker image that is around 10MB with the backend code also hosting the static frontend chunks.

