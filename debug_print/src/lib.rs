use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn debug_print(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree as an expression
    let input_expr = parse_macro_input!(input as Expr);

    // Get the string representation of the input expression
    let input_expr_string = input_expr.to_token_stream().to_string();

    // Generate the code to print the value and type of the input expression
    let debug_code = quote! {
        println!("{} = {:?}", #input_expr_string, #input_expr);
    };

    // Convert the generated code into a TokenStream
    TokenStream::from(debug_code) // debug_code.into()
}
