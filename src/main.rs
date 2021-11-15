use base64::decode;
use linked_hash_map::LinkedHashMap;
use std::io;
use std::io::Write;
use std::str;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

/// Get part of a yaml, based on a &str key.
fn get_yaml_part_of<'a, 'b>(src: &'a Yaml, key: &'b str) -> &'a Yaml {
    src.as_hash()
        .expect("cannot properly read yaml as_hash")
        .iter()
        .find(|(k, _)| k.as_str().expect("key could not be read/converted as_str") == key) // search key
        .expect("expected data key block to be present")
        .1 // take value of "data" (i.e. the hashmap)
}

// Replace a certain block in a YAML, identified by a key, with a new/replacement block
fn replace_in_yaml(orig: &Yaml, replace_key: &str, replace_val: &Yaml) -> Yaml {
    // Construct new yaml, with our decoded values
    let mut out_all = LinkedHashMap::new();
    for (k, v) in orig.as_hash().expect("cannot properly read yaml as_hash") {
        match k {
            Yaml::String(k_str) if k_str == replace_key => {
                out_all.insert(k.clone(), replace_val.clone())
            }
            _ => out_all.insert(k.clone(), v.clone()),
        };
    }
    Yaml::Hash(out_all)
}

// Decode a &str and return a new String
fn base64_decode(src: &str) -> String {
    let decoded = &decode(src).expect("could not decode base64 string");
    String::from(str::from_utf8(decoded).expect("could not construct str from_utf8"))
}

fn strip_crlf_inplace(src: &mut String) {
    if src.ends_with('\n') {
        src.pop();
        if src.ends_with('\r') {
            src.pop();
        }
    }
}

fn decode_k8s_secret_yaml_to(yaml_str: &str) -> String {
    // Load our string yaml we read from stdin
    let yaml_docs = YamlLoader::load_from_str(yaml_str).expect("could not load yaml from &str");
    let yaml_doc = &yaml_docs[0]; // just take the first one...
    let data_key = "data"; // this is the block in the yaml doc we want to replace

    let mut dec_data_lhm = LinkedHashMap::new();
    let data_block = get_yaml_part_of(yaml_doc, data_key);
    data_block
        .as_hash() // as a hash
        .expect("could not get data section as_hash")
        .iter()
        .for_each(|(k, v)| {
            let mut decoded_val = base64_decode(v.as_str().expect("could not read as &str"));
            strip_crlf_inplace(&mut decoded_val);
            dec_data_lhm.insert(k.clone(), Yaml::from_str(&decoded_val));
        });
    let decoded_data_block = Yaml::Hash(dec_data_lhm);

    // Construct new yaml, with our decoded values
    let decoded_out_yaml = replace_in_yaml(yaml_doc, data_key, &decoded_data_block);

    // Write yaml to tmp buffer string
    let mut out_str_emitter = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str_emitter);
    emitter
        .dump(&decoded_out_yaml)
        .expect("could not write output YAML to buffer (String)"); // dump the YAML object to a String

    // Clean output before return
    out_str_emitter
        .chars()
        .into_iter()
        .skip(4) // skip "---" and skip "\n"
        .collect::<String>()
}

fn main() {
    // Read input from stdin
    let mut input_buf = String::new();
    loop {
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line) {
            Ok(len) => {
                if len == 0 || &input_line == "\n" || &input_line == "\r" || &input_line == "\r\n" {
                    break;
                }
                input_buf.push_str(input_line.as_str());
                input_line.clear();
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
    if input_buf.len() == 0 {
        return;
    }

    // Do magic
    let out_string_cleaned = decode_k8s_secret_yaml_to(&input_buf);
    // Write to stdout
    io::stdout()
        .lock()
        .write_all(out_string_cleaned.as_bytes())
        .expect("could not write to stdout");
}

/// https://kubernetes.io/docs/concepts/configuration/secret/
#[cfg(test)]
mod tests {
    use super::*;

    fn get_secret_1() -> String {
        let secret = r#"
        apiVersion: v1
        kind: Secret
        metadata:
          name: secret-sa-sample
          annotations:
            kubernetes.io/service-account.name: "sa-name"
        type: kubernetes.io/service-account-token
        data:
          extra: YmFyCg==
        "#;
        String::from(secret)
    }

    fn get_secret_1_dec() -> String {
        let secret = r#"
        apiVersion: v1
        kind: Secret
        metadata:
          name: secret-sa-sample
          annotations:
            kubernetes.io/service-account.name: "sa-name"
        type: kubernetes.io/service-account-token
        data:
          extra: bar
        "#;
        String::from(secret)
    }

    fn get_secret_2() -> String {
        let secret = r#"
        apiVersion: v1
        kind: Secret
        metadata:
          name: bootstrap-token-5emitj
          namespace: kube-system
        type: bootstrap.kubernetes.io/token
        data:
          auth-extra-groups: c3lzdGVtOmJvb3RzdHJhcHBlcnM6a3ViZWFkbTpkZWZhdWx0LW5vZGUtdG9rZW4=
          expiration: MjAyMC0wOS0xM1QwNDozOToxMFo=
          token-id: NWVtaXRq
          token-secret: a3E0Z2lodnN6emduMXAwcg==
          usage-bootstrap-authentication: dHJ1ZQ==
          usage-bootstrap-signing: dHJ1ZQ==
        "#;
        String::from(secret)
    }

    fn get_secret_3() -> String {
        let secret = r#"
        apiVersion: v1
        kind: Secret
        metadata:
          name: secret-ssh-auth
        type: kubernetes.io/ssh-auth
        data:
          cargo.toml: |
            W3BhY2thZ2VdCm5hbWUgPSAiZGVrcyIKdmVyc2lvbiA9ICIwLjEuMCIKZWRpdGlvbiA9ICIyMDIxIgoKIyBTZWUgbW9yZSBrZXlzIGFuZCB0aGVpciBkZWZpbml0aW9ucyBhdCBodHRwczovL2RvYy5ydXN0LWxhbmcub3JnL2NhcmdvL3JlZmVyZW5jZS9tYW5pZmVzdC5odG1sCgpbZGVwZW5kZW5jaWVzXQp5YW1sLXJ1c3QgPSAiMC40IgpiYXNlNjQgPSAiMC4xMy4wIgpsaW5rZWQtaGFzaC1tYXAgPSAiMC41LjQi"#;
        String::from(secret)
    }

    #[test]
    fn test_get_yaml_part_of_1() {
        let yaml_docs =
            YamlLoader::load_from_str(&get_secret_1()).expect("could not load yaml from &str");
        let yaml_doc = &yaml_docs[0]; // just take the first one...

        let mut exp_tmp = LinkedHashMap::new();
        exp_tmp.insert(
            Yaml::String(String::from("extra")),
            Yaml::String(String::from("YmFyCg==")),
        );
        let exp_yaml = Yaml::Hash(exp_tmp);

        assert_eq![&exp_yaml, get_yaml_part_of(yaml_doc, "data")];
    }

    #[test]
    fn test_get_yaml_part_of_2() {
        let yaml_docs =
            YamlLoader::load_from_str(&get_secret_2()).expect("could not load yaml from &str");
        let yaml_doc = &yaml_docs[0]; // just take the first one...

        let mut exp_tmp = LinkedHashMap::new();
        exp_tmp.insert(
            Yaml::String(String::from("auth-extra-groups")),
            Yaml::String(String::from(
                "c3lzdGVtOmJvb3RzdHJhcHBlcnM6a3ViZWFkbTpkZWZhdWx0LW5vZGUtdG9rZW4=",
            )),
        );
        exp_tmp.insert(
            Yaml::String(String::from("expiration")),
            Yaml::String(String::from("MjAyMC0wOS0xM1QwNDozOToxMFo=")),
        );
        exp_tmp.insert(
            Yaml::String(String::from("token-id")),
            Yaml::String(String::from("NWVtaXRq")),
        );
        exp_tmp.insert(
            Yaml::String(String::from("token-secret")),
            Yaml::String(String::from("a3E0Z2lodnN6emduMXAwcg==")),
        );
        exp_tmp.insert(
            Yaml::String(String::from("usage-bootstrap-authentication")),
            Yaml::String(String::from("dHJ1ZQ==")),
        );
        exp_tmp.insert(
            Yaml::String(String::from("usage-bootstrap-signing")),
            Yaml::String(String::from("dHJ1ZQ==")),
        );
        let exp_yaml = Yaml::Hash(exp_tmp);

        assert_eq![&exp_yaml, get_yaml_part_of(yaml_doc, "data")];
    }

    #[test]
    fn test_get_yaml_part_of_3() {
        let yaml_docs =
            YamlLoader::load_from_str(&get_secret_3()).expect("could not load yaml from &str");
        let yaml_doc = &yaml_docs[0]; // just take the first one...

        let mut exp_tmp = LinkedHashMap::new();
        exp_tmp.insert(
            Yaml::String(String::from("cargo.toml")),
            Yaml::String(String::from("W3BhY2thZ2VdCm5hbWUgPSAiZGVrcyIKdmVyc2lvbiA9ICIwLjEuMCIKZWRpdGlvbiA9ICIyMDIxIgoKIyBTZWUgbW9yZSBrZXlzIGFuZCB0aGVpciBkZWZpbml0aW9ucyBhdCBodHRwczovL2RvYy5ydXN0LWxhbmcub3JnL2NhcmdvL3JlZmVyZW5jZS9tYW5pZmVzdC5odG1sCgpbZGVwZW5kZW5jaWVzXQp5YW1sLXJ1c3QgPSAiMC40IgpiYXNlNjQgPSAiMC4xMy4wIgpsaW5rZWQtaGFzaC1tYXAgPSAiMC41LjQi")),
        );
        let exp_yaml = Yaml::Hash(exp_tmp);

        assert_eq![&exp_yaml, get_yaml_part_of(yaml_doc, "data")];
    }

    #[test]
    fn test_replace_in_yaml() {
        let yaml_docs =
            YamlLoader::load_from_str(&get_secret_1()).expect("could not load yaml from &str");
        let yaml_doc = &yaml_docs[0]; // just take the first one...

        let mut repl_tmp = LinkedHashMap::new();
        repl_tmp.insert(
            Yaml::String(String::from("extra")),
            Yaml::String(String::from("Zm9v==")), // replace the encoded "bar" with an encoded "foo"
        );
        let replacement_yaml = Yaml::Hash(repl_tmp);

        let replaced_doc = replace_in_yaml(yaml_doc, "data", &replacement_yaml);

        assert_eq![&replacement_yaml, get_yaml_part_of(&replaced_doc, "data")];
    }

    #[test]
    fn test_decode_k8s_secret_yaml_to_1() {
        assert_eq![
            YamlLoader::load_from_str(&get_secret_1_dec()),
            YamlLoader::load_from_str(&decode_k8s_secret_yaml_to(&get_secret_1()))
        ];
    }
}
