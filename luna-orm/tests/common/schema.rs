/*
use storage_generator::schema::schema_raw::SchemaRaw;

pub fn get_test_schema() -> SchemaRaw {
    let content = r#"
---
name: article

fields:
- name: id
  type: uuid
- name: author_id
  type: uuid
- name: title
  type: string
- name: content
  type: string
- name: last_update_time
  type: timestamp


index_infos:
  primary:
    partition:
    - id
    clustering:
    - title
  secondary:
  - name: time_index
    fields:
    - last_update_time
  - name: author_id_title_index
    fields:
    - author_id
    - title
method_infos:
  has_default_method: true
  methods: []
"#;
    let schema_raw: SchemaRaw = serde_yaml::from_str(content).unwrap();
    return schema_raw;
}
*/
