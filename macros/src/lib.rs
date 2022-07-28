use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;
use syn::Ident;
use syn::ItemStruct;
use syn::LitStr;
use syn::{Field, Fields::Named, FieldsNamed, Path, Type, TypePath};

#[proc_macro_derive(ValidateCompare)]
pub fn validate_compare(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let fields = if let Named(FieldsNamed { ref named, .. }) = &item.fields {
        named
    } else {
        panic!("We couldn't get the struct fields?");
    };

    let struct_name = &item.ident;
    //let fn_name = struct_name.to_string() + "Builder";

    let data = StructMetaData {
        name: format!("{}", item.ident),
        fields: fields.iter().filter_map(|field| get_field_medatada(field)).collect(),
    };

    let fields: Vec<String> = data.fields.iter().map(|field| format!("{}(test.{}.clone())", field.name.to_string(), field.name.to_string())).collect();
    let print_str = fields.join(".");
    let result = quote! {
        impl #struct_name {
            pub fn print_field_names() {
                let fields = ::std::string::String::from(#print_str);
                println!("{}", fields);
            }
        }
    };
    result.into()
}

#[proc_macro_attribute]
pub fn json_validate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let filename = parse_macro_input!(attr as LitStr);

    let struct_name = &item.ident;

    let prefix_string = struct_name.to_string().to_uppercase();
    let schema_name = prefix_string.clone() + "_SCHEMA";
    let schema_name = Ident::new(&schema_name, Span::call_site());

    let json_name = prefix_string.clone() + "_JSON";
    let json_name = Ident::new(&json_name, Span::call_site());

    let validator_name = prefix_string + "_VALIDATOR";
    let validator_name = Ident::new(&validator_name, Span::call_site());

    let result = quote! {
        const #schema_name: &str = include_str!(#filename);

        lazy_static! {
            static ref #json_name: serde_json::Value = serde_json::from_str(#schema_name).expect(&format!("Invalid Schema File Format: {}", #filename));
            static ref #validator_name: jsonschema::JSONSchema = jsonschema::JSONSchema::compile(&#json_name).expect(&format!("Invalid Schema File: {}", #filename));
        }

        impl validation_macros::JsonValidate for #struct_name {

            fn schema_validate(&self) -> Result<(), validation_macros::JsonValidateError> {
                use tracing::{warn, trace};

                if let Err(val) = #validator_name.validate(&serde_json::json!(self)).map_err(|errors| validation_macros::JsonValidateError::ValidationError(Vec::from_iter(errors.map(|e| e.to_string())))){
                    warn!("Validate failed on Json Value Struct {:?}, with error: {} ", &self, val);
                    return Err(val);
                } else {
                    trace!("Succesfully validated Json Value Struct {:?}", &self);
                    return Ok(());
                }
            }
        }
        #item
    };
    result.into()
}

#[derive(Debug)]
struct StructMetaData {
    name: String,
    fields: Vec<FieldMetaData>,
}

#[derive(Debug)]
struct FieldMetaData {
    name: String,
    ty: String,
}

// Sample to assist writing above macro.
// DELETEABLE WHEN MACRO IS FINISHED
// mod test {
//     use super::*;
//     use validation_macros::JsonValidate;
//     use test_strategy::proptest;

//     /// Test validation via builder against validation via schema
//     #[proptest]
//     fn compare_request_builder_validation_with_schema_validation(proptest_struct: super::BootNotificationRequest) {
//         let v = proptest_struct.clone();
//         let built_struct = BootNotificationRequestBuilder::default()
//             .charge_point_vendor(v.charge_point_vendor)
//             .charge_point_model(v.charge_point_model.clone())
//             .charge_point_serial_number(v.charge_point_serial_number)
//             .charge_box_serial_number(v.charge_box_serial_number)
//             .firmware_version(v.firmware_version)
//             .iccid(v.iccid)
//             .imsi(v.imsi)
//             .meter_type(v.meter_type)
//             .meter_serial_number(v.meter_serial_number)
//             .build();

//         let builder_validated_ok = built_struct.is_ok();
//         let schema_validated_ok = proptest_struct.schema_validate().is_ok();
//         assert_eq!(builder_validated_ok, schema_validated_ok);
//     }

//     #[proptest]
//     fn compare_response_builder_validation_with_schema_validation(proptest_struct: super::BootNotificationResponse) {
//         let v = proptest_struct.clone();
//         let built_struct = BootNotificationResponseBuilder::default().status(v.status).current_time(v.current_time).interval(v.interval).build();

//         let builder_validated_ok = built_struct.is_ok();
//         let schema_validated_ok = proptest_struct.schema_validate().is_ok();
//         assert_eq!(builder_validated_ok, schema_validated_ok);
//     }
// }

fn get_field_medatada(field: &Field) -> Option<FieldMetaData> {
    let ident = match &field.ident {
        Some(id) => Some(format!("{}", id)),
        None => {
            return None;
        }
    };

    let ty_ident = match &field.ty {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) => segments.first().and_then(|s| Some(format!("{}", s.ident))),
        _ => {
            return None;
        }
    };
    let entity_field = FieldMetaData { name: ident.unwrap(), ty: ty_ident.unwrap() };
    Some(entity_field)
}
