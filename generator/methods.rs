use std::ops::Add;

use crate::common::get_good_field_name;

use super::{common, spec_types};
use common::{create_file, IMP_URL};
use convert_case::{Casing, Case};

pub fn create_import_crate(obj: &spec_types::MethodDescription) -> (String, bool) {
    let mut has_input_file = false;
    let (mut impdata, import_array) = match &obj.fields {
        Some(fields) => {
            let mut import_array: Vec<String> = Vec::new();
            for field in fields {
                if !common::is_dtype_builtin(&field.types[0]) {
                    let impname = field.types[0].replace("Array of ", "");
                    if impname == "InputFile" {
                        has_input_file = true;
                        continue;
                    }
                    if obj.name == impname {
                        continue;
                    }
                    if import_array.contains(&impname) {
                        continue;
                    }
                    // use crate::types::PhotoSize
                    import_array.push(impname);
                }
            }
            let mut imptxt = String::new();
            if has_input_file {
                imptxt = format!("use std::collections::HashMap;\nuse {IMP_URL}::input_file::InputFile;\n");
            }
            if import_array.len() == 0 {
                (imptxt, import_array)
            } else {
                imptxt = imptxt.add(format!("use {IMP_URL}::types::").as_str());
                if import_array.len() == 1 {
                    (imptxt.add(import_array[0].as_str()).add(";\n"), import_array)
                } else {
                    let mut num = 0;
                    for imp in import_array.iter() {
                        num += 1;
                        if num == 1 {
                            imptxt = imptxt.add(format!("{{{}", imp).as_str());
                            continue;
                        }
                        imptxt = imptxt.add(format!(", {}", imp).as_str())
                    }
                    (imptxt.add("};\n"), import_array)
                }   
            }
        },
        None => (String::new(), Vec::new()),
    };
    if obj.returns.len() > 0 && !common::is_dtype_builtin(&obj.returns[0]) {
        let mut found = false;
        for imptype in import_array.iter() {
            if &obj.returns[0] == imptype {
                found = true;
                break;
            }
        }
        if !found {
            impdata = impdata.add(format!("use {IMP_URL}::types::{};\n", &obj.returns[0].replace("Array of ", "")).as_str());
        }
    }
    (impdata, has_input_file)
}

pub async fn generate_methods(spec: &spec_types::ApiDescription) {
    let mut data = String::new();
    for (_, method) in spec.methods.iter() {
        let (good_tname, builder_name) = generate_method(method).await;
        data = data.add(format!("\nmod {};\npub use {}::{};\n", good_tname, good_tname, builder_name).as_str());
    }
    create_file(String::from("methods/mod.rs"), data);
}

async fn generate_method(method: &spec_types::MethodDescription) -> (String, String) {
    let name = &method.name.to_case(Case::Snake); 
    let mut data = String::from(common::WARNING_COMMENT);
    // don't warn on too many arguments
    data = data.add("#![allow(clippy::too_many_arguments)]\n");
    data = data.add("use serde::Serialize;\n\n");
    data = data.add(format!("use {IMP_URL}::Bot;\n").as_str());
    data = data.add(format!("use {IMP_URL}::error::Result;\n").as_str());
    let (imptxt, has_input_file)  = create_import_crate(method);
    data = data.add(imptxt.as_str());
    // data = data.add(format!("\n/// <{}>", method.href).as_str());
    let builder_name = format!("{}Builder", &method.name.to_case(Case::UpperCamel)); 
    data = data.add(generate_bot_imp(&method, &builder_name).as_str());
    data = data.add(generate_builder(&method, &builder_name, has_input_file).as_str());
    data = data.add(generate_imp(&method, &builder_name, has_input_file).as_str());
    create_file(String::from(format!("methods/{}.rs", &name)), data);
    (name.clone(), builder_name)
}

// impl Bot {
//     pub fn send_message(&self, chat_id: i64, text: String) -> SendMessageBuilder {
//         SendMessageBuilder::new(&self, chat_id, text)
//     }
// }
fn generate_bot_imp(method: &spec_types::MethodDescription, builder_name: &String) -> String {
    let mut args = String::new();
    let mut vals = String::new();
    let method_name = method.name.to_case(Case::Snake);
    let mut generics = String::new();
    let mut generic_ret = String::new();
    if method.fields.is_some() {
        for field in method.fields.as_ref().unwrap() {
            let mut field_type = field.types[0].clone();
            if field_type == "InputFile" {
                generics = String::from("<F: InputFile>");
                field_type = String::from("F");
                generic_ret = String::from("<F>");
            }
            if !field.required {
                continue;
            }
            let field_name = get_good_field_name(&field.name);
            
            let dtype = common::get_type(&common::get_data_type(&field_type), field.required, false);
            args = args.add(format!(", {field_name}: {dtype}").as_str());
            vals = vals.add(format!(", {field_name}").as_str())
        }
    }
    let mut desc = String::new();
    if let Some(md) = method.description.as_ref()  {
        for d in md.iter() {
            desc = desc.add(format!("\n    /// {}", d).as_str())
        }
    }
    desc = desc.add(format!("\n    /// <{}>", &method.href).as_str());
    let data = format!("
impl Bot {{{desc}
    pub fn {method_name}{generics}(&self{args}) -> {builder_name}{generic_ret} {{
        {builder_name}::new(self{vals})
    }}
}}

");
    data
}

fn generate_builder(method: &spec_types::MethodDescription, builder_name: &String, has_input_file: bool) -> String {
    let mut data = String::new();
    let generic = match has_input_file {
        true => "<'a, F: InputFile>",
        false => "<'a>",
    };
    data = data.add(format!("#[derive(Serialize)]
pub struct {builder_name}{generic} {{
    #[serde(skip)]
    bot: &'a Bot,").as_str());
    if has_input_file {
        data = data.add("
    #[serde(skip)]
    data: HashMap<&'a str, F>,");
    }
    data = data.add(generate_fields(method).as_str());
    data = data.add("\n}\n\n");
    data
}

fn generate_imp(method: &spec_types::MethodDescription, builder_name: &String, has_input_file: bool) -> String {
    let (generic_def, generic_use, data_attr, req_method) = match has_input_file {
        true => ("<'a, F: InputFile>", "<'a, F>", ", Some(self.data)", "post"),
        false => ("<'a>", "<'a>", "", "get"),
    };
    let mut insertion_defs = String::new();
    let mut is_map_mut = false;
    let (new_fn_attr, new_fn_cntnt, builder_fns) = match &method.fields {
        Some(fields) => {
            let mut builder_fns = String::new();
            let mut new_fn_attr = String::new();
            let mut new_fn_cntnt = String::new();
            for field in fields.iter() {
                let field_name = get_good_field_name(&field.name);
                let field_type = field.types[0].clone();
                let dtype = common::get_type(&common::get_data_type(&field_type), field.required, false);
                let field_value;
                let mut rtype = common::get_type(&common::get_data_type(&field_type), true, false);
                if field.required {
                    if field_type == "InputFile" {
                        is_map_mut = true;
                        field_value = field_name.clone();
                        new_fn_attr = new_fn_attr.add(format!(", {}: F", field_name).as_str());
                        insertion_defs = insertion_defs.add(format!("data.insert(\"{field_value}\", {field_value});\n").as_str());
                        rtype = String::from("F");
                    } else {
                        field_value = field_name.clone();
                        new_fn_attr = new_fn_attr.add(format!(", {}: {}", field_name, dtype).as_str());
                        new_fn_cntnt = new_fn_cntnt.add(format!("\n            {},", field_name).as_str());
                    }
                } else {
                    if field_type != "InputFile" {
                        field_value = format!("Some({})", field_name);
                        new_fn_cntnt = new_fn_cntnt.add(format!("\n            {}: None,", field_name).as_str());
                    } else {
                        field_value = field_name.clone();
                        rtype = String::from("F");
                    }
                }
                let builder_fn_data = match field_type == "InputFile" {
                    true => format!("self.data.insert(\"{field_name}\", {field_value});"),
                    false => format!("self.{field_name} = {field_value};"),
                }; 
                builder_fns = builder_fns.add(format!("
    pub fn {field_name}(mut self, {field_name}: {rtype}) -> Self {{
        {builder_fn_data}self
    }}
                ").as_str());
            }
        (new_fn_attr, new_fn_cntnt, builder_fns)
        },
        None => {
            (String::new(), String::new(), String::new())
        }
    };
    let (predef_attr, extra_defs) = match has_input_file {
        true => {
            let hashmap_decl: String;
            if is_map_mut {
                hashmap_decl = String::from("let mut data = HashMap::new();\n");
            } else {
                hashmap_decl = String::from("let data = HashMap::new();\n")
            }
            ("bot, data,", hashmap_decl.add(&insertion_defs))
        },
        false => ("bot,", String::new())
    };
    let builder_fns = builder_fns.add(format!("
    pub async fn send(self) -> Result<{to_return}> {{
        let form = serde_json::to_value(&self)?;
        self.bot.{req_method}(\"{raw_method_name}\", Some(&form){data_attr}).await
    }}
", to_return=common::get_type(&common::get_data_type(&method.returns[0]), true, false), raw_method_name=method.name).as_str());
    let impl_new_fn_string = format!("
impl{generic_def} {builder_name}{generic_use} {{
    pub fn new(bot: &'a Bot{new_fn_attr}) -> Self {{
        {extra_defs}Self{{
            {predef_attr}{new_fn_cntnt}
        }}
    }}
{builder_fns}
}}");
    impl_new_fn_string
}

fn generate_fields(obj: &spec_types::MethodDescription) -> String {
    match &obj.fields {
        Some(fields) => {
            let mut generated_fields_string = String::new();
            for field in fields.iter() {
                let mut field_type = field.types[0].clone();
                if field_type == "InputFile" {
                    continue;
                }
                if obj.name == field_type || (obj.name == "Chat" && field_type == "Message") || obj.name == "Message" && field_type == "Chat" {
                    field_type = format!("Box<{}>", field_type)
                }
                generated_fields_string = generated_fields_string.add(format!("\n    /// {}", field.description).as_str());
                if !field.required {
                    generated_fields_string = generated_fields_string.add(format!("\n    #[serde(skip_serializing_if = \"Option::is_none\")]").as_str());
                }
                generated_fields_string = generated_fields_string.add(format!("\n    pub {name}: {dtype},",name=common::get_good_field_name(&field.name), dtype=common::get_type(&common::get_data_type(&field_type), field.required, false)).as_str())
            }
            generated_fields_string
        },
        None => String::new(),
    }
}