extern crate proc_macro;

use std::ffi;

#[proc_macro]
pub fn static_cstr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = syn::parse_macro_input!(input as syn::LitStr);

    let cstring = match ffi::CString::new(literal.value()) {
        Ok(x) => x,
        Err(err) => {
            return syn::Error::new(proc_macro::Span::call_site().into(), err)
                .to_compile_error()
                .into();
        }
    };

    let expr = syn::LitByteStr::new(
        cstring.as_bytes_with_nul(),
        proc_macro::Span::call_site().into(),
    );

    quote::quote!({
        union Cast {
            from_byte_slice: &'static [u8],
            to_cstr: &'static CStr,
        }

        unsafe {
            Cast { from_byte_slice: #expr }.to_cstr
        }
    })
    .into()
}
