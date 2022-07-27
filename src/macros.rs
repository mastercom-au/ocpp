//! QoL macros 

#[macro_export]
/// Expands to the builder for a particular OCPP structure
macro_rules! generate_builders {
    ($i:expr) => {
        paste::paste!{            
        impl [<$i RequestBuilder>] {
            /// Builder function for [<$i>] struct
            pub fn build(&self) -> Result<[<$i Request>], OcppError> {
                let req = self.pre_build()?;
                return req.validate().map(|_| req).map_err(|e| e.into());
            }
        }
        impl [<$i ResponseBuilder>] {
            /// Builder function for [<$i>] struct
            pub fn build(&self) -> Result<[<$i Response>], OcppError> {
                let req = self.pre_build()?;
                return req.validate().map(|_| req).map_err(|e| e.into());
            }
        }
        }

    };
}