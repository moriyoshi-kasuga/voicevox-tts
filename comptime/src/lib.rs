#[proc_macro]
pub fn all_voices(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let text = include_str!("../../voicevox_core/model/metas.json");

    let voices: Vec<VoiceCharacter> = serde_json::from_str(text).unwrap();

    let (character, id): (Vec<String>, Vec<u32>) =
        voices.into_iter().fold((vec![], vec![]), |mut acc, v| {
            for style in v.styles {
                if style.r#type.is_none() {
                    acc.0.push(format!("{}-{}", v.name, style.name));
                    acc.1.push(style.id);
                }
            }

            acc
        });

    quote::quote! {
        [
            #(
                (#character,#id)
            ),*
        ]
    }
    .into()
}

#[derive(serde::Deserialize)]
struct VoiceCharacter {
    name: &'static str,
    styles: Vec<VoiceStyles>,
}

#[derive(serde::Deserialize)]
struct VoiceStyles {
    name: &'static str,
    id: u32,
    r#type: Option<&'static str>,
}
