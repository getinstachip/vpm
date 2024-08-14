pub mod versions {

    use anyhow::Result;
    use std::fs::File;
    use std::io::Write;
    use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value};

    const DEFAULT_LIB_NAME: &str = "default_library";
    const DEFAULT_LIB_VERSION: &str = "0.1.0";
    const DEFAULT_LIB_DESCRIPTION: &str = "A default library";
    const DEFAULT_VERSION: &str = "0.1.0";
    const DEFAULT_BRANCH: &str = "main";

    const VPM_TOML: &str = "vpm.toml";
    const VPM_LOCK: &str = "vpm.lock";

    pub fn create_toml(is_lock: bool) -> Result<DocumentMut> {
        let mut doc = DocumentMut::new();
        let mut lib = Table::new();
        lib.insert("name", Item::Value(Value::from(DEFAULT_LIB_NAME)));
        lib.insert("version", Item::Value(Value::from(DEFAULT_LIB_VERSION)));
        lib.insert("description", Item::Value(Value::from(DEFAULT_LIB_DESCRIPTION)));
        lib.insert("authors", Item::Value(Value::Array(Array::new())));
        lib.insert("license", Item::Value(Value::Array(Array::new())));
        lib.insert("include", Item::Value(Value::Array(Array::new())));
        doc.insert("library", Item::Table(lib));

        doc.insert("docs", Item::Table(Table::new()));
        doc.insert("config", Item::Table(Table::new()));
        if is_lock {
            doc.insert("lock-dependencies", Item::Table(Table::new()));
        } else {
            doc.insert("dependencies", Item::Table(Table::new()));
            doc.insert("dev-dependencies", Item::Table(Table::new()));
        }

        Ok(doc)
    }

    pub fn update_library_entry(doc: &mut DocumentMut,
                            lib_name: Option<&str>,
                            lib_version: Option<&str>,
                            lib_description: Option<&str>,
                            lib_authors: Option<&str>,
                            lib_license: Option<&str>,
                            lib_include: Option<&str>
                            ) -> Result<()> {

        let lib = doc.entry("library").or_insert(Item::Table(Table::new())).as_table_mut().unwrap();

        if lib_name.unwrap_or("") != "" {
            let mut _name_entry = lib.get_mut("name").unwrap();
            _name_entry = & mut Item::Value(Value::from(lib_name.unwrap()));
        }

        if lib_version.unwrap_or("") != "" {
            let mut _version_entry = lib.get_mut("version").unwrap();
            _version_entry = & mut Item::Value(Value::from(lib_version.unwrap()));
        }

        if lib_description.unwrap_or("") != "" {
            let mut _description_entry = lib.get_mut("description").unwrap();
            _description_entry = & mut Item::Value(Value::from(lib_description.unwrap()));
        }

        if lib_authors.unwrap_or("") != "" {
            let mut _authors_entry = lib.get_mut("authors").unwrap();
            let mut authors = Array::new();
            for author in lib_authors.unwrap().split(", ").collect::<Vec<&str>>() {
                authors.push(Value::from(author));
            }
            _authors_entry = & mut Item::Value(Value::Array(authors));
        }

        if lib_license.unwrap_or("") != "" {
            let mut _license_entry = lib.get_mut("license").unwrap();
            let mut license = Array::new();
            for license_pair in lib_license.unwrap().split(", ").collect::<Vec<&str>>() {
                let pair = license_pair.split(": ").collect::<Vec<&str>>();
                let mut table = InlineTable::new();
                table.get_or_insert("type", Value::from(pair[0]));
                table.get_or_insert("source", Value::from(pair[1]));
                license.push(table);
            }
            _license_entry = & mut Item::Value(Value::Array(license));
        }

        if lib_include.unwrap_or("") != "" {
            let mut _include_entry = lib.get_mut("include").unwrap();
            let mut include = Array::new();
            for include_path in lib_include.unwrap().split(", ").collect::<Vec<&str>>() {
                include.push(Value::from(include_path));
            }
            _include_entry = & mut Item::Value(Value::Array(include));
        }

        Ok(())

    }

    pub fn update_config_entry(doc: &mut DocumentMut,
                            section_name: &str,
                            variable_name: &str,
                            variable_value: Value
                            ) -> Result<()> {

        let docs = doc.entry(section_name).or_insert(Item::Table(Table::new())).as_table_mut().unwrap();
        if docs.contains_key(variable_name) {
            let mut _item = docs.get_mut(variable_name).unwrap();
            _item = & mut Item::Value(variable_value);
        } else {
            docs.insert(variable_name, Item::Value(variable_value));
        }

        Ok(())

    }

    pub fn update_dependencies_entry(doc: &mut DocumentMut,
                                    section_name: &str,
                                    uri: &str,
                                    version: Option<&str>,
                                    alias: Option<&str>,
                                    modules: Option<Vec<String>>,
                                    branch: Option<&str>,
                                    commit: Option<&str>
                                    ) -> Result<()> {
        
        let deps = doc.entry(section_name).or_insert(Item::Table(Table::new())).as_table_mut().unwrap();
        if deps.contains_key(uri) {
            let table = deps.get_mut(uri).unwrap().as_table_mut().unwrap();
            if version.unwrap_or("") != "" {
                let mut _version = table.get_mut("version").unwrap();
                _version = & mut Item::Value(Value::from(version.unwrap()));
            }
            if alias.unwrap_or("") != "" {
                let mut _alias = table.get_mut("alias").unwrap();
                _alias = & mut Item::Value(Value::from(alias.unwrap()));
            }
            if modules.clone().unwrap_or(vec![]).len() > 0 {
                let current_modules = table.get_mut("modules").unwrap().as_array_mut().unwrap();
                for module in modules.unwrap() {
                    if module == "" || current_modules.clone().into_iter().any(|m| m.as_str().unwrap() == module) { continue; }
                    current_modules.push(Value::from(module));
                }
            }
            if branch.unwrap_or("") != "" {
                let mut _branch = table.get_mut("branch").unwrap();
                _branch = & mut Item::Value(Value::from(branch.unwrap()));
            }
            if commit.unwrap_or("") != "" {
                let mut _commit = table.get_mut("commit").unwrap();
                _commit = & mut Item::Value(Value::from(commit.unwrap()));
            }
        } else {
            let mut table = InlineTable::new();
            table.insert("version", Value::from(version.unwrap_or(DEFAULT_VERSION)));
            if alias.unwrap_or("") != "" { table.insert("alias", Value::from(alias.unwrap())); }
            if modules.clone().unwrap_or(vec![]).len() > 0 {
                let mut _modules = Array::new();
                for module in modules.unwrap() {
                    _modules.push(Value::from(module));
                }
                table.insert("modules", Value::Array(_modules));
            }
            if branch.unwrap_or("") != "" { table.insert("branch", Value::from(branch.unwrap())); }
            table.insert("branch", Value::from(branch.unwrap_or(DEFAULT_BRANCH)));
            if commit.unwrap_or("") != "" { table.insert("commit", Value::from(commit.unwrap())); }

            deps.insert(uri, Item::Value(Value::InlineTable(table)));
        }

        Ok(())

    }

    pub fn write_file(doc: DocumentMut, is_lock: bool) -> Result<()> {
        let toml_str = doc.to_string();
        let file_ext = if is_lock {"lock"} else {"toml"};
        let mut toml_file = File::create(if is_lock {VPM_LOCK} else {VPM_TOML}).expect(&format!("Failed to create vpm.{file_ext}"));
        toml_file.write_all(toml_str.as_bytes()).expect(&format!("Failed to write to vpm.{file_ext}"));
        Ok(())
    }

    pub fn read_file(is_lock: bool) -> Result<DocumentMut> {
        let file_ext = if is_lock {"lock"} else {"toml"};
        let toml_str = std::fs::read_to_string(if is_lock {VPM_TOML} else {VPM_LOCK}).expect(&format!("Failed to read vpm.{file_ext}. Try running `vpm init` first."));
        let doc = toml_str.parse::<DocumentMut>().expect(&format!("Failed to parse vpm.{file_ext}"));
        Ok(doc)
    }

}