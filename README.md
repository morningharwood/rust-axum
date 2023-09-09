# rust-axum
A rust axum fly.io deployment


errors from flyclt

```
==> Pushing image to fly
The push refers to repository [registry.fly.io/rust-axum]
f12200739669: Pushed
db5be1f4adfd: Layer already exists
faa6cc1ed213: Layer already exists
6e3b92711bf1: Layer already exists
deployment-01H9XTPYV092S0DMXSN58E1MNJ: digest: sha256:b709e3260a4785af8ac504b47e0a7ebb6bd713bbffee43b59098218ef3f7a0e3 size: 1154
--> Pushing image done
image: registry.fly.io/rust-axum:deployment-01H9XTPYV092S0DMXSN58E1MNJ
image size: 87 MB

Watch your deployment at https://fly.io/apps/rust-axum/monitoring

Updating existing machines in 'rust-axum' with rolling strategy
  Machine 5683d514b3028e [app] has state: started
  [1/1] Checking that 5683d514b3028e [app] is up and running
Smoke checks for 5683d514b3028e failed: the app appears to be crashing
Check its logs: here's the last lines below, or run 'fly logs -i 5683d514b3028e':
  Successfully prepared image registry.fly.io/rust-axum:deployment-01H9XTPYV092S0DMXSN58E1MNJ (1.297391483s)
  Configuring firecracker
  [    0.040115] PCI: Fatal: No config space access function found
   INFO Starting init (commit: 5293a085)...
   INFO Preparing to run: `./rust-axum` as app
   INFO [fly api proxy] listening at /.fly/api
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by ./rust-axum)
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by ./rust-axum)
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.34' not found (required by ./rust-axum)
  2023/09/09 20:39:28 listening on [fdaa:2:fc5f:a7b:1af:bfe0:82e3:2]:22 (DNS: [fdaa::3]:53)
   INFO Main child exited normally with code: 1
   INFO Starting clean up.
   WARN hallpass exited, pid: 256, status: signal: 15 (SIGTERM)
  2023/09/09 20:39:29 listening on [fdaa:2:fc5f:a7b:1af:bfe0:82e3:2]:22 (DNS: [fdaa::3]:53)
  [    2.289057] reboot: Restarting system
  machine did not have a restart policy, defaulting to restart
  [    0.040112] PCI: Fatal: No config space access function found
   INFO Starting init (commit: 5293a085)...
   INFO Preparing to run: `./rust-axum` as app
   INFO [fly api proxy] listening at /.fly/api
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by ./rust-axum)
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by ./rust-axum)
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.34' not found (required by ./rust-axum)
  2023/09/09 20:39:31 listening on [fdaa:2:fc5f:a7b:1af:bfe0:82e3:2]:22 (DNS: [fdaa::3]:53)
   INFO Main child exited normally with code: 1
   INFO Starting clean up.
   WARN hallpass exited, pid: 256, status: signal: 15 (SIGTERM)
  2023/09/09 20:39:32 listening on [fdaa:2:fc5f:a7b:1af:bfe0:82e3:2]:22 (DNS: [fdaa::3]:53)
  [    2.287922] reboot: Restarting system
  machine did not have a restart policy, defaulting to restart
  [    0.040185] PCI: Fatal: No config space access function found
   INFO Starting init (commit: 5293a085)...
   INFO Preparing to run: `./rust-axum` as app
   INFO [fly api proxy] listening at /.fly/api
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by ./rust-axum)
  ./rust-axum: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by ./rust-axum)
  2023/09/09 20:39:33 listening on [fdaa:2:fc5f:a7b:1af:bfe0:82e3:2]:22 (DNS: [fdaa::3]:53)
Error: smoke checks for 5683d514b3028e failed: the app appears to be crashing
```
