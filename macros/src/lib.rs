extern crate proc_macro;
use nom::IResult;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::fs::read_dir;

/// Looks in the puzzle director for all files matching `day_*.rs` and imports them as modules.
#[proc_macro]
pub fn import_solutions(_item: TokenStream) -> TokenStream {
    let imports = TokenStream2::from_iter(get_days().into_iter().map(|day| {
        let module_name = format_ident!("day_{:02}", day);
        let type_name = format_ident!("Day{:02}", day);
        proc_macro2::TokenStream::from(quote! {
            mod #module_name;
            pub use #module_name::#type_name;
        })
    }));

    imports.into()
}

#[proc_macro]
pub fn get_solution(item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);

    let solutions = TokenStream2::from_iter(get_days().into_iter().map(|day| {
        let type_name = format_ident!("Day{:02}", day);
        proc_macro2::TokenStream::from(quote! {
            #day => Box::new(#type_name::new(#item)),
        })
    }));

    quote! {
        match #item.day as u32 {
            #solutions
            _ => anyhow::bail!("No solution found for day {} (src/puzzle/day_{:02}.rs)", #item.day, #item.day),
        }
    }
    .into()
}

fn get_days() -> Vec<u32> {
    read_dir("./src/puzzle")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let filename = entry.file_name().into_string().unwrap();
            let Ok((_, day)) = parse_day_from_file(&filename) else {
                return None;
            };
            Some(day)
        })
        .filter(|d| *d > 0)
        .collect()
}

fn parse_day_from_file(filename: &str) -> IResult<&str, u32> {
    let (input, _) = nom::bytes::complete::tag("day_")(filename)?;
    let (input, day) = nom::character::complete::digit1(input)?;
    let day: u32 = day.parse().unwrap();

    Ok((input, day))
}
