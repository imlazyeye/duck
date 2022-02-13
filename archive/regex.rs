
/// Parses a body of text for all GmlEnums.
fn collect_gml_enums_from_gml_regex(contents: &str, collection: &mut Vec<GmlEnum>) {
    // Get enums out of the contents
    for captures in Regex::new(r"enum (\w+) \{([\n\s#\w,/*]+)\}")
        .unwrap()
        .captures_iter(contents)
    {
        let mut gml_enum = GmlEnum::new(captures.get(1).unwrap().as_str().to_string());
        let enum_body = captures.get(2).unwrap().as_str();
        let entry_finder = Regex::new(r"(\w+),").unwrap();

        // Parse through the enum declaration line by line
        for line in enum_body.lines() {
            // Are we at the end?
            if line.contains('}') {
                break;
            }
            // Is this an entry, and not some random junk?
            if entry_finder.is_match(line) {
                // Each line of entries could be MULTIPLE entries (ex: `MemberOne, MemberTwo, MemberThree`)
                for cap in entry_finder.captures_iter(line) {
                    gml_enum.add_member(cap.get(1).unwrap().as_str().to_string());
                }
            }
        }

        // Collect it!
        collection.push(gml_enum);
    }
}

fn collect_gml_switch_statements_from_gml_regex(
    resource_path: &Path,
    contents: &str,
    collection: &mut Vec<GmlSwitchStatement>,
) {
    fn collect_switches(
        contents: &str,
        switch_finder: &Regex,
        clippie_style_default_finder: &Regex,
        resource_path: &Path,
        collection: &mut Vec<GmlSwitchStatement>,
    ) {
        if let Some(captures) = switch_finder.captures(contents) {
            let full_body = captures.get(0).unwrap().as_str();
            let cases_body = captures.get(1).unwrap();

            // Does this switch contain nested switches?
            if let Some(inner_captures) = switch_finder.captures(cases_body.as_str()) {
                // It does! Let's collect this guy all buy himself.
                let inner_full_body = inner_captures.get(0).unwrap().as_str();
                collect_switches(
                    inner_full_body,
                    switch_finder,
                    clippie_style_default_finder,
                    resource_path,
                    collection,
                );

                // Cool, now let's retry with that guy removed.
                collect_switches(
                    &contents.replace(inner_full_body, ""),
                    switch_finder,
                    clippie_style_default_finder,
                    resource_path,
                    collection,
                )
            } else {
                // No nesting! We're good to go.
                // Idenitfy the type of this switch statement
                if let Some(enum_name_capture) =
                    clippie_style_default_finder.captures(cases_body.as_str())
                {
                    // Create the switch
                    let mut gml_switch = GmlSwitchStatement::new(
                        enum_name_capture.get(1).unwrap().as_str().to_string(),
                        resource_path.to_str().unwrap().to_string(),
                    );

                    // Go over the body of the cases and register each case
                    let case_finder = Regex::new(r"case ([\w\.\d]+):").unwrap();
                    for case_literal in cases_body
                        .as_str()
                        .lines()
                        .flat_map(|line| case_finder.captures_iter(line))
                        .map(|captures| captures.get(1))
                        .flatten()
                        .map(|re_match| re_match.as_str())
                    {
                        // The literal being checked for in this case
                        gml_switch.add_case(case_literal.to_string());
                    }

                    // Okay, register!
                    collection.push(gml_switch);
                } else {
                    // There is a default, but its not set up for clippie. We ignore these!
                }

                // Now we recurse with ourselves removed...
                collect_switches(
                    &contents.replace(full_body, ""),
                    switch_finder,
                    clippie_style_default_finder,
                    resource_path,
                    collection,
                )
            }
        }
    }

    // First, let's just get a count of how many clippie-style default cases we can find.
    // Later on, we're going to use this to ensure we parsed all the enums we needed to.
    let clippie_style_default_finder =
        Regex::new(r#"default: IMPOSSIBLE\("Unexpected (\w+)"#).unwrap();
    let expected_matches = clippie_style_default_finder.captures_iter(contents).count();
    let start_count = collection.len();

    // Make our pattern
    let switch_finder = Regex::new(r#"switch.+\n((?:.|\n)+?default:(?:.+))"#).unwrap();

    let contents = contents.to_string();
    collect_switches(
        &contents,
        &switch_finder,
        &clippie_style_default_finder,
        resource_path,
        collection,
    );

    // Ensure that the number of matches we just got is what we expected
    let found_count = collection.len() - start_count;
    if found_count != expected_matches {
        let delta = expected_matches - found_count;
        warn!("Failed to parse {delta} switch statements in '{resource_path:?}'!");
    }
}

fn check_for_missing_default_cases_in_gml(resource_path: &Path, contents: &str) {
    let missing_default_finder = Regex::new(r#"switch(?:.|\n)+?(?:default|switch|$)"#).unwrap();
    for captures in missing_default_finder.captures_iter(contents) {
        warn!(
            target: &Self::create_path_for_captures(resource_path, contents, captures),
            "Missing default case on switch statement.",
        );
    }
}
