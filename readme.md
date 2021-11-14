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
* `kubectl get secret db-user-pass |dks`
* `kubectl get secret db-user-pass |dks |less` or 
* `cat <k8s-secret.yaml> |dks > my-decoded-k8s-secret.yml` or ...

# Tests
Rudimentary tests are available. Run `cargo test` to see the results...

# FAQ
* Is it _fast_? **Probably not. At least not as fast as it could be, but then again, how big is a given k8s secret...**