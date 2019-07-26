# Google Local API Proxy

This small http proxy creates authentication free Google Local API endpoints from newer Local API endpoints
(see https://github.com/rithvikvibhu/GHLocalApi/issues/39).

This can be used to fix up automations (e.g. via home-assistants [google home support](https://www.home-assistant.io/components/googlehome/)) until the new api is better understood.

The server understands the following environment variables:

Variable      | Usage                                                | Default
--------------|------------------------------------------------------|----------------
`SOURCE_IP`   | IP of the google home to be addressed by the proxy   | None <required>
`SOURCE_PORT` | Port of the google home to be addressed by the proxy | 8443
`LISTEN_IP`   | IP to listen for requests on                         | 0.0.0.0
`LISTEN_PORT` | Port to listen for requests on                       | 8008
`TOKEN`       | HomeGraph-Token used to authenticate                 | None <required>

You currently need a rooted android device to optain the required token.
See: https://gist.github.com/rithvikvibhu/1a0f4937af957ef6a78453e3be482c1f

## Building

This is a [rust](https://rust-lang.org) project. Build using `cargo`:
`cargo build --release`

## Docker

The server is published at [docker hub](https://hub.docker.com/r/drakulix/ghlapi_proxy) for easy deployment.