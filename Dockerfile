FROM scratch
COPY target/x86_64-unknown-linux-musl/release/ghlapi_proxy /ghlapi_proxy
ENTRYPOINT [ "/ghlapi_proxy" ]
