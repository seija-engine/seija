use syn::*;
use uuid::Uuid;
use quote::quote;

#[proc_macro_derive(TypeUuid, attributes(uuid))]
pub fn type_uuid_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let mut uuid:Option<Uuid> = None;
    for attribute in ast.attrs.iter().filter_map(|attr| attr.parse_meta().ok()) {
        let name_value = if let Meta::NameValue(name_value) = attribute {
            name_value
        } else {
            continue;
        };
        if name_value.path.get_ident().map(|i| i != "uuid").unwrap_or(true) {
            continue;
        };
        let uuid_str = match name_value.lit {
            Lit::Str(lit_str) => lit_str,
            _ => panic!("uuid attribute must take the form `#[uuid = \"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx\"`"),
        };
        uuid = Some(Uuid::parse_str(&uuid_str.value()).expect("Value specified to `#[uuid]` attribute is not a valid UUID"));
    };
    let uuid= uuid.expect("No `#[uuid = \"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx\"` attribute found");
    let bytes = uuid.as_bytes().iter().map(|byte| format!("{:#X}", byte)).map(|byte_str| syn::parse_str::<LitInt>(&byte_str).unwrap());
    let gen = quote! {
        impl TypeUuid for #name {
            const TYPE_UUID: Uuid = Uuid::from_bytes([
                #( #bytes ),*
            ]);
        }
    };
    gen.into()
}
