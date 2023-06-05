FROM node:20.2.0-alpine
COPY frontend /workspace
WORKDIR /workspace
RUN apk add g++ make py3-pip && npm install && npm run build

FROM rust:alpine
COPY backend /workspace
WORKDIR /workspace
RUN apk add --no-cache musl-dev && cargo build --release

FROM alpine:latest
COPY --from=0 /workspace/dist /workspace/static
COPY --from=1 /workspace/target/release/online-clipboard /workspace/online-clipboard
EXPOSE 8000
CMD ["/workspace/online-clipboard"]