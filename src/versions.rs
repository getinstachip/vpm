pub mod versions {

    use anyhow::Result;
    use std::io::Write;
    use std::fs::{OpenOptions, read_to_string};
    use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value};

    const DEFAULT_LIB_NAME: &str = "default_library";
    const DEFAULT_LIB_VERSION: &str = "0.1.0";
    const DEFAULT_LIB_DESCRIPTION: &str = "A default library";
    const DEFAULT_VERSION: &str = "0.1.0";
    const DEFAULT_BRANCH: &str = "main";

    const VPM_TOML: &str = "vpm.toml";
    const VPM_LOCK: &str = "vpm.lock";

    pub fn create_toml(is_lock: bool) -> Result<()> {
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

        write_file(doc, is_lock)?;

        Ok(())
    }

    pub fn update_library_entry(is_lock: bool,
                                lib_name: Option<&str>,
                                lib_version: Option<&str>,
                                lib_description: Option<&str>,
                                lib_authors: Option<&str>,
                                lib_license: Option<&str>,
                                lib_include: Option<&str>
                                ) -> Result<()> {
        
        let mut doc = read_file(is_lock)?;
        let lib = doc.entry("library").or_insert(Item::Table(Table::new())).as_table_mut().unwrap();
        
        if lib_name.unwrap_or("") != "" {
            lib.insert("name", Item::Value(Value::from(lib_name.unwrap())));
        }

        if lib_version.unwrap_or("") != "" {
            lib.insert("version", Item::Value(Value::from(lib_version.unwrap())));
        }

        if lib_description.unwrap_or("") != "" {
            lib.insert("description", Item::Value(Value::from(lib_description.unwrap())));
        }

        if lib_authors.unwrap_or("") != "" {
            let mut authors = Array::new();
            for author in lib_authors.unwrap().split(", ").collect::<Vec<&str>>() {
                authors.push(Value::from(author));
            }
            lib.insert("authors", Item::Value(Value::Array(authors)));
        }

        if lib_license.unwrap_or("") != "" {
            let mut license = Array::new();
            for license_pair in lib_license.unwrap().split(", ").collect::<Vec<&str>>() {
                let pair = license_pair.split(": ").collect::<Vec<&str>>();
                let mut table = InlineTable::new();
                table.get_or_insert("type", Value::from(pair[0]));
                table.get_or_insert("source", Value::from(pair[1]));
                license.push(table);
            }
            lib.insert("license", Item::Value(Value::Array(license)));
        }

        if lib_include.unwrap_or("") != "" {
            let mut include = Array::new();
            for include_path in lib_include.unwrap().split(", ").collect::<Vec<&str>>() {
                include.push(Value::from(include_path));
            }
            lib.insert("include", Item::Value(Value::Array(include)));
        }

        write_file(doc, is_lock)?;

        Ok(())

    }

    pub fn update_config_entry(is_lock: bool,
                               section_name: &str,
                               variable_name: &str,
                               variable_value: Value
                               ) -> Result<()> {

        let mut doc = read_file(is_lock)?;
        let docs = doc.entry(section_name).or_insert(Item::Table(Table::new())).as_table_mut().unwrap();
        docs.insert(variable_name, Item::Value(variable_value));

        write_file(doc, is_lock)?;

        Ok(())

    }

    pub fn update_dependencies_entry(is_lock: bool,
                                     section_name: &str,
                                     uri: &str,
                                     version: Option<&str>,
                                     alias: Option<&str>,
                                     modules: Option<Vec<String>>,
                                     branch: Option<&str>,
                                     commit: Option<&str>
                                     ) -> Result<()> {
        
        let mut doc = read_file(is_lock)?;
        let deps = doc.entry(section_name).or_insert(Item::Table(Table::new())).as_table_mut().unwrap();
        if deps.contains_key(uri) {
            let table = deps.entry(uri).or_insert(Item::Table(Table::new())).as_table_mut().unwrap();
            if version.unwrap_or("") != "" {
                table.insert("version", Item::Value(Value::from(version.unwrap())));
            }
            if alias.unwrap_or("") != "" {
                table.insert("alias", Item::Value(Value::from(alias.unwrap())));
            }
            if modules.clone().unwrap_or(vec![]).len() > 0 {
                let current_modules = table.entry("modules").or_insert(Item::Value(Value::Array(Array::new()))).as_array_mut().unwrap();
                for module in modules.unwrap() {
                    if module == "" || current_modules.clone().into_iter().any(|m| m.as_str().unwrap() == module) { continue; }
                    current_modules.push(Value::from(module));
                }
            }
            if branch.unwrap_or("") != "" {
                table.insert("branch", Item::Value(Value::from(branch.unwrap())));
            }
            if commit.unwrap_or("") != "" {
                table.insert("commit", Item::Value(Value::from(commit.unwrap())));
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

        write_file(doc, is_lock)?;

        Ok(())

    }

    fn write_file(doc: DocumentMut, is_lock: bool) -> Result<()> {
        let toml_str = doc.to_string();
        let file_ext = if is_lock {"lock"} else {"toml"};
        let mut file = OpenOptions::new().write(true)
                                                    .create(true)
                                                    .truncate(true)
                                                    .open(if is_lock {VPM_LOCK} else {VPM_TOML})
                                                    .expect(&format!("Failed to open vpm.{file_ext}"));
        file.write_all(toml_str.as_bytes()).expect(&format!("Failed to write to vpm.{file_ext}"));
        Ok(())
    }

    fn read_file(is_lock: bool) -> Result<DocumentMut> {
        let file_ext = if is_lock {"lock"} else {"toml"};
        let toml_str = read_to_string(if is_lock {VPM_LOCK} else {VPM_TOML}).expect(&format!("Failed to read vpm.{file_ext}. Try running `vpm init` first."));
        let doc = toml_str.parse::<DocumentMut>().expect(&format!("Failed to parse vpm.{file_ext}"));
        Ok(doc)
    }

}