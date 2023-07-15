// use anyhow::Result;
// use std::path::PathBuf;

// use crate::steamcmd::workshop::{WorkshopCollection, WorkshopItem};

// pub fn from_file(file: impl Into<PathBuf>) -> Result<WorkshopCollection> {
//     let file = file.into();

//     let document = std::fs::read_to_string(&file)?;

//     from_string(&document)
// }

// pub fn from_string(document: &str) -> Result<WorkshopCollection> {
//     let document = scraper::Html::parse_document(document);

//     if !validate(&document) {
//         return Err(anyhow::anyhow!("Invalid preset file"));
//     }

//     // // extract name from <meta name="arma:PresetName" content="RHS Remastered" />
//     let name = get_name(&document);

//     // // <div class="mod-list"> ... </div>
//     let mods = document
//         .select(&scraper::Selector::parse("div.mod-list").unwrap())
//         .next()
//         .unwrap();

//     // // list of <tr data-type="ModContainer">
//     let mods = mods
//         .select(&scraper::Selector::parse("tr[data-type=\"ModContainer\"]").unwrap())
//         .collect::<Vec<_>>();

//     let mut collection = WorkshopCollection::new(name);

//     for mod_ in mods {
//         let id = get_mod_id(&mod_);
//         let name = get_mod_name(&mod_);

//         collection.insert(WorkshopItem {
//             name,
//             app_id: super::CLIENT_APP_ID,
//             published_file_id: id,
//             time_updated: 0,
//             path: None,
//         });
//     }

//     Ok(collection)
// }

fn get_mod_id(element: &scraper::ElementRef) -> u64 {
    // <a href="https://steamcommunity.com/sharedfiles/filedetails/?id=450814997" data-type="Link">https://steamcommunity.com/sharedfiles/filedetails/?id=450814997</a>
    let link = element
        .select(&scraper::Selector::parse("a[data-type=\"Link\"]").unwrap())
        .next()
        .unwrap();

    let href = link.value().attr("href").unwrap();

    let id = href.split('/').last().unwrap(); // ?id=450814997
    let id = id.split('=').last().unwrap(); // 450814997
    let id = id.parse::<u64>().unwrap(); // 450814997

    id
}

fn get_mod_name(element: &scraper::ElementRef) -> String {
    // <td data-type="DisplayName">RHSUSAF</td>
    let name = element
        .select(&scraper::Selector::parse("td[data-type=\"DisplayName\"]").unwrap())
        .next()
        .unwrap();

    name.text().collect::<String>()
}

fn get_name(document: &scraper::Html) -> String {
    document
        .select(&scraper::Selector::parse("meta[name=\"arma:PresetName\"]").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("content")
        .unwrap()
        .to_owned()
}

fn validate(document: &scraper::Html) -> bool {
    validate_inner(document, "arma:Type", Some("preset"))
        && validate_inner(document, "generator", Some("Arma 3 Launcher - https://arma3.com"))
        && validate_inner(document, "arma:PresetName", None)
}

fn validate_inner(document: &scraper::Html, name: &str, value: Option<&str>) -> bool {
    let Some(value) = value else {
        // only check for existence
        return document.select(&scraper::Selector::parse(&format!("meta[name=\"{}\"]", name)).unwrap()).next().is_some();
    };

    // get the meta tag where name == arma:Type
    let Some(meta_type) = document.select(&scraper::Selector::parse(&format!("meta[name=\"{}\"]", name)).unwrap()).next() else {
        return false;
    };

    // get the value of the content attribute
    let Some(meta_type) = meta_type.value().attr("content") else {
        return false;
    };

    // check if the value is equal to "Preset"
    if meta_type != value {
        return false;
    }

    true
}
