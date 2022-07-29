use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemStruct, LitStr};

#[proc_macro_derive(ValidateCompare)]
pub fn validate_compare(item: TokenStream) -> TokenStream {
    // Parse attached item as ItemStruct
    let item = parse_macro_input!(item as ItemStruct);
    // Grab struct name and fields
    let struct_identifier = item.ident.clone();
    let field_names: Vec<Ident> = item.fields.iter().filter_map(|field| field.ident.clone()).map(|ident| ident).collect();
    // Builder name is specified by builder_derive as $struct_name + Builder
    let builder_name = format_ident!("{}Builder", struct_identifier);

    let result = quote! {
        impl #struct_identifier {
            pub fn test_build(fuzz_struct: Self) -> bool {
                let built_struct = #builder_name ::default()#(.#field_names(fuzz_struct.#field_names .clone()))* .build();
                let builder_validated_ok = built_struct.is_ok();
                let schema_validated_ok = fuzz_struct.schema_validate().is_ok();
                return builder_validated_ok == schema_validated_ok;
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

    let json_name = format_ident!("{}_JSON", prefix_string);
    let schema_name = format_ident!("{}_SCHEMA", prefix_string);
    let validator_name = format_ident!("{}_VALIDATOR", prefix_string);

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
