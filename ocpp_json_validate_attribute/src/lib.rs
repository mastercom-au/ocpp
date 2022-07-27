use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;
use syn::Ident;
use syn::ItemStruct;
use syn::LitStr;

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

        impl ocpp_json_validate::JsonValidate for #struct_name {

            fn schema_validate(&self) -> Result<(), ocpp_json_validate::JsonValidateError> {
                use tracing::{warn, trace};

                if let Err(val) = #validator_name.validate(&serde_json::json!(self)).map_err(|errors| ocpp_json_validate::JsonValidateError::ValidationError(Vec::from_iter(errors.map(|e| e.to_string())))){
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
