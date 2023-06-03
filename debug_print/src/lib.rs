use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, parse::Parse};

struct MyInput(String);

impl Parse for MyInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<Expr>()?;
        let expr = expr.to_token_stream().to_string();
        Ok(
            MyInput(expr)
        )
    }
}

impl ToTokens for MyInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn debug_print(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree as an expression
    let input_expr = parse_macro_input!(input as MyInput);

    // Get the string representation of the input expression
    let input_expr_string = input_expr.0.clone();

    // Generate the code to print the value and type of the input expression
    let debug_code = quote! {
        println!("{} = {:?}", #input_expr_string, #input_expr);
    };

    // Convert the generated code into a TokenStream
    TokenStream::from(debug_code) // debug_code.into()
}
