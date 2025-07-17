# kcp-sys

Safe bindings to the [kcp](https://github.com/skywind3000/kcp) transport protocol library.

Also including a high level API for connection state management and data stream handling.

## Usage

1. Create the endpoint and run it.

    ```rust
    let mut endpoint = KcpEndpoint::new();
    endpoint.run().await;
    ```

2. forward the input and output to your transport layer, udp for example.

    ```rust
    let (input, mut output) = (endpoint.input_sender(), endpoint.output_receiver().unwrap());

    let udp_socket = Arc::new(UdpSocket::bind("0.0.0.0:54320").await.unwrap());
    udp_socket.connect("127.0.0.1:54321").await.unwrap();

    let udp = udp_socket.clone();
    tokio::spawn(async move {
        while let Some(data) = output.recv().await {
            udp.send(&data.inner()).await.unwrap();
        }
    });

    let udp = udp_socket.clone();
    tokio::spawn(async move {
        loop {
            let mut buf = vec![0; 1024];
            let (size, _) = udp.recv_from(&mut buf).await.unwrap();
            input
                .send(BytesMut::from(&buf[..size]).into())
                .await
                .unwrap();
        }
    });
    ```
4. Create a connection and send / recv data.

    ```rust
    let conn_id = endpoint
        .connect(Duration::from_secs(1), 0, 0, Bytes::new())
        .await
        .unwrap();

    let mut kcp_stream = KcpStream::new(&endpoint, conn_id).unwrap();
    kcp_stream.write_all(b"hello world").await.unwrap();

    let mut buf = vec![0; 64 * 1024];
    let size = kcp_stream.read(&mut buf).await.unwrap();

    println!("{}", String::from_utf8_lossy(&buf[..size]));
    ```

## Tune the kcp parameters

You can tune the kcp parameters by set a config factory to the endpoint.

```rust
let mut endpoint = KcpEndpoint::new();
endpoint.set_kcp_config_factory(|conv| {
    KcpConfig::new_turbo(conv)
});
```

## Building

This crate uses pregenerated FFI bindings to avoid requiring LLVM/Clang at build time.

### For Users

Simply run:
```bash
cargo build
```

No additional dependencies required! The pregenerated bindings are included in the source code.

### For Maintainers

When the KCP submodule is updated, you need to regenerate the bindings:

```bash
# Generate new bindings for the current KCP version
cargo build --features bindgen

# Commit the generated bindings file
git add src/binding_*.rs
git commit -m "Update bindings for KCP version"
```

The generated binding file is named `binding_{commit_id}.rs` where `commit_id` is the current KCP submodule commit hash.

### Build Features

- **Default**: Uses pregenerated bindings, no LLVM dependency
- **`bindgen`**: Enables bindgen to generate new bindings when needed
