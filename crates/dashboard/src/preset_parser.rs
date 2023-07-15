use api_schema::{
    request::{CreatePresetSchema, PresetItemSchema},
    response::PresetItem,
};

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

    let mut index = 0;
    for mod_ in mods {
        index += 1;
        let id = get_mod_id(&mod_);
        let name = get_mod_name(&mod_);

        items.push(PresetItemSchema {
            name,
            published_file_id: id,
            position: index,
            enabled: true,
        });
    }

    Ok(CreatePresetSchema { name, items })
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
    let id = id.parse::<i64>().unwrap(); // 450814997

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
