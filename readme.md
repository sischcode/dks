# About
`dks` stands for "Decode Kubernetes Secret" and does just that.

It's purely written to bypass the offical/recommended way of doing, e.g. `kubectl get secrets/db-user-pass --template={{.data.password}} | base64 -D` or `kubectl get secret db-user-pass -o json | jq '.data | map_values(@base64d)'` with, in this example, `jq` (json query). Sometimes you just want to see the whole secret without remembering all this.

`dks` decodes the base64 encoded part of a Kubernetes Secret, which is the `data` block in the YAML, and then returns/outputs the complete YAML with the decoded `data` block. 

I.e. 
```
apiVersion: v1
kind: Secret
metadata:
  name: secret-sa-sample
  annotations:
    kubernetes.io/service-account.name: "sa-name"
type: kubernetes.io/service-account-token
data:
  extra: YmFyCg==
```
will become:
```
apiVersion: v1
kind: Secret
metadata:
  name: secret-sa-sample
  annotations:
    kubernetes.io/service-account.name: "sa-name"
type: kubernetes.io/service-account-token
data:
  extra: bar
```
Which is, the inputted secret, but with a decoded `data` block.

# Usage
* `kubectl get secret db-user-pass -o yaml |dks`
* `kubectl get secret db-user-pass -o yaml |dks |less` or 
* `cat <k8s-secret.yaml> |dks > my-decoded-k8s-secret.yml` or ...

# Building from src
## For Windows and Linux with Rust already installed.
1. `git clone https://github.com/sischcode/dks.git`
2. `cd dks`
3. `cargo build --release`

Enjoy your binary at `dks/target/release/dks`

## For various platforms - no Rust installation required
### Windows (x86_64)
(windows-gnu is compatible with the GCC/MinGW ABI)
1. `git clone https://github.com/sischcode/dks.git`
2. `cd dks`
3. `docker build -t dks:0.2.0-x86_64-pc-windows-gnu -f build/x86_64-pc-windows-gnu/Dockerfile .`
4. `docker create -ti --name dks_build dks:0.2.0-x86_64-pc-windows-gnu bash && docker cp dks_build:/dks/build/dks_windows_x86_64.tar.gz . && docker rm -f dks_build`

### Linux (x86_64)
1. `git clone https://github.com/sischcode/dks.git`
2. `cd dks`
3. `docker build -t dks:0.2.0-x86_64-unknown-linux-gnu -f build/x86_64-unknown-linux-gnu/Dockerfile .`
4. `docker create -ti --name dks_build dks:0.2.0-x86_64-unknown-linux-gnu bash && docker cp dks_build:/dks/build/dks_linux_x86_64.tar.gz . && docker rm -f dks_build`


# Tests
Rudimentary tests are available. Run `cargo test` to see the results...

# FAQ
* Is it _fast_? **Probably not. At least not as fast as it could be, but then again, how big is a given k8s secret...**
