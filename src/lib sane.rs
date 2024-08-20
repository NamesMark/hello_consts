extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;

macro_rules! process_pow {
    ($num:expr, $buf:expr, $pow:expr, $idx:expr) => {{
        if $num >= ($pow as i32) {
            get_sub_1000(&mut $buf, ($num / ($pow as i32)) as i32);
            $buf.push(POW_10[$idx].to_string());
            $num %= $pow as i32;
        }
    }};
}

#[proc_macro]
pub fn generate_named_constants(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let (start, end) = if let Expr::Range(range) = expr {
        (
            if let Some(from) = range.start {
                if let Expr::Lit(expr_lit) = *from {
                    if let Lit::Int(lit_int) = expr_lit.lit {
                        lit_int.base10_parse::<i32>().unwrap()
                    } else {
                        panic!("Range start is not an integer literal");
                    }
                } else {
                    panic!("Range start is not a literal");
                }
            } else {
                panic!("Range does not have a start");
            },
            if let Some(to) = range.end {
                if let Expr::Lit(expr_lit) = *to {
                    if let Lit::Int(lit_int) = expr_lit.lit {
                        lit_int.base10_parse::<i32>().unwrap()
                    } else {
                        panic!("Range end is not an integer literal");
                    }
                } else {
                    panic!("Range end is not a literal");
                }
            } else {
                panic!("Range does not have an end");
            },
        )
    } else {
        panic!("Input is not a range expression");
    };

    let mut output = quote! {};
    for i in start..=end {
        let name = number_to_words(i);
        let ident = format_ident!("{}", name);
        output = quote! {
            #output
            const #ident: i32 = #i;
        };
    }

    output.into()
}

fn number_to_words(mut num: i32) -> String {
    if num == 0 {
        return SUB_20[0].to_string();
    }

    let mut buf: Vec<String> = Vec::new();

    process_pow!(num, buf, 1e9, 3); // billions
    process_pow!(num, buf, 1e6, 2); // millions
    process_pow!(num, buf, 1e3, 1); // thousands

    // process below 1000:
    get_sub_1000(&mut buf, num);

    buf.join("").to_lowercase()
}

const SUB_20: [&str; 20] = [
    "Zero",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];

const SUB_100: [&str; 10] = [
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

const POW_10: [&str; 4] = ["Hundred", "Thousand", "Million", "Billion"];

fn get_sub_1000(buf: &mut Vec<String>, mut num: i32) {
    if num >= 100 {
        buf.push(SUB_20[(num / 100) as usize].to_string());
        buf.push(POW_10[0].to_string());
        num %= 100;
    }

    if num >= 20 {
        buf.push(SUB_100[(num / 10) as usize].to_string());
        num %= 10;
    }

    if num > 0 {
        buf.push(SUB_20[num as usize].to_string());
    }
}
