use api_schema::request::{CreatePresetSchema, PresetDlcSchema, PresetItemSchema};

pub fn is_preset(document: &str) -> bool {
    let document = scraper::Html::parse_document(document);

    validate(&document)
}

pub fn parse(document: &str) -> Result<CreatePresetSchema, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(document);

    let name = get_name(&document);

    let mods = document
        .select(&scraper::Selector::parse("div.mod-list").unwrap())
        .next()
        .ok_or("Invalid preset file")?;

    // // list of <tr data-type="ModContainer">
    let mods = mods
        .select(&scraper::Selector::parse("tr[data-type=\"ModContainer\"]").unwrap())
        .collect::<Vec<_>>();

    let mut items = Vec::default();

    for (index, mod_) in mods.into_iter().enumerate() {
        let index = index + 1;
        let id = get_mod_id(&mod_);
        let name = get_mod_name(&mod_);

        items.push(PresetItemSchema {
            name,
            published_file_id: id,
            position: index as i64,
            enabled: true,
        });
    }

    let dlcs = document
        .select(&scraper::Selector::parse("div.dlc-list").unwrap())
        .next()
        .ok_or("Invalid preset file")?;

    // // list of <tr data-type="DlcContainer">
    let dlcs = dlcs
        .select(&scraper::Selector::parse("tr[data-type=\"DlcContainer\"]").unwrap())
        .collect::<Vec<_>>();

    let mut dlc_items = vec![
        PresetDlcSchema {
            name: String::from("Spearhead 1944"),
            key: String::from("spe"),
            app_id: 1175380,
            enabled: false,
            position: 1,
        },
        PresetDlcSchema {
            name: String::from("Western Sahara"),
            key: String::from("ws"),
            app_id: 1681170,
            enabled: false,
            position: 2,
        },
        PresetDlcSchema {
            name: String::from("S.O.G. Prairie Fire"),
            key: String::from("vn"),
            app_id: 1227700,
            enabled: false,
            position: 3,
        },
        PresetDlcSchema {
            name: String::from("CSLA Iron Curtain"),
            key: String::from("csla"),
            app_id: 1294440,
            enabled: false,
            position: 4,
        },
        PresetDlcSchema {
            name: String::from("Global Mobilization"),
            key: String::from("gm"),
            app_id: 1042220,
            enabled: false,
            position: 5,
        },
        PresetDlcSchema {
            name: String::from("Contact"),
            key: String::from("enoch"),
            app_id: 1021790,
            enabled: false,
            position: 6,
        },
    ];

    for dlc in dlcs {
        let app_id = get_mod_id(&dlc);

        // find the DLC in the list from app_id and set enabled to true
        for dlc_item in &mut dlc_items {
            if dlc_item.app_id == app_id {
                dlc_item.enabled = true;
            }
        }
    }

    Ok(CreatePresetSchema {
        name,
        items,
        dlcs: dlc_items,
    })
}

fn get_mod_id(element: &scraper::ElementRef) -> i64 {
    // <a href="https://steamcommunity.com/sharedfiles/filedetails/?id=450814997" data-type="Link">https://steamcommunity.com/sharedfiles/filedetails/?id=450814997</a>
    let link = element
        .select(&scraper::Selector::parse("a[data-type=\"Link\"]").unwrap())
        .next()
        .unwrap();

    let href = link.value().attr("href").unwrap();

    let id = href.split('/').last().unwrap(); // ?id=450814997
    let id = id.split('=').last().unwrap(); // 450814997
    id.parse::<i64>().unwrap() // 450814997
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
        return document
            .select(&scraper::Selector::parse(&format!("meta[name=\"{}\"]", name)).unwrap())
            .next()
            .is_some();
    };

    // get the meta tag where name == arma:Type
    let Some(meta_type) = document
        .select(&scraper::Selector::parse(&format!("meta[name=\"{}\"]", name)).unwrap())
        .next()
    else {
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
