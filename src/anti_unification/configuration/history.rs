use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generaliser::generaliser::Generaliser;
#[derive(Clone, Debug)]
pub struct History {
    configs : Vec<(Configuration,String)>,
}


impl History {
    pub fn new() -> Self {
        Self { configs: vec![]}
    }

    pub fn from_vec(configs: &Vec<(Configuration,String)>) -> Self {
        Self { configs: configs.clone()}
    }

    pub fn add_config(&mut self, c: &Configuration, rule: &str) {
        self.configs.push((c.clone(),rule.to_string()));
    }




    /// Generates a DOT format string for graph visualization
    pub fn to_dot(&self) -> String
    where
        Configuration: Display,
    {
        let mut dot = String::new();
        let n = self.configs.len();
        // Graph header with vertical layout
        dot.push_str("digraph ComputationHistory {\n");
        dot.push_str("    rankdir=TB; // Top to bottom layout\n");
        dot.push_str("    node [shape=rectangle, style=rounded];\n");
        dot.push_str("    edge [fontsize=10];\n\n");

        // Add nodes (configurations) - nodes are labeled with the Configuration's Display
        for (i, (config, _)) in self.configs.iter().enumerate() {
            let node_label = self.escape_dot_string(&config.to_string());
            dot.push_str(&format!("    config{} [label=\"{}\"];\n", i, node_label));
        }

        //Add the generaliser at the end
        if let Some(last_config) = self.configs.last() {
            let node_label = self.escape_dot_string(&last_config.0.to_generaliser().to_string());
            dot.push_str(&format!("    config{} [label=\"{}\"];\n", n, node_label));
        }


        dot.push_str("\n");

        // Add edges (transitions with labels) - edges are labeled with the String from the tuple
        for (i, (_, arrow_label)) in self.configs.iter().enumerate().take(self.configs.len() - 1) {
            let escaped_label = self.escape_dot_string(arrow_label);
            dot.push_str(&format!("    config{} -> config{} [label=\"{}\"];\n",
                                  i, i + 1, escaped_label));
        }
        //Last edge
        dot.push_str(&format!("    config{} -> config{};\n", n-1, n));

        dot.push_str("}\n");
        dot
    }

    /// Escapes special characters for DOT format
    fn escape_dot_string(&self, s: &str) -> String {
        s.replace('\"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\\', "\\\\")
    }

    /// Writes the DOT representation to a file
    pub fn write_dot_file(&self, filename: &str) -> std::io::Result<()>
    where
        Configuration: Display,
    {
        let mut file = File::create(filename)?;
        file.write_all(self.to_dot().as_bytes())?;
        Ok(())
    }

    /// Generates an image from the DOT file using Graphviz
    /// Returns Result indicating success or failure
    pub fn generate_image(&self, dot_filename: &str, output_filename: &str, format: &str) -> Result<(), String>
    where
        Configuration: Display,
    {
        // First, write the DOT file
        if let Err(e) = self.write_dot_file(dot_filename) {
            return Err(format!("Failed to write DOT file: {}", e));
        }

        // Use Graphviz to generate the image
        let output = Command::new("dot")
            .args(&["-T", format, "-o", output_filename, dot_filename])
            .output()
            .map_err(|e| format!("Failed to execute dot command: {}. Is Graphviz installed?", e))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Graphviz failed: {}", error_msg));
        }

        Ok(())
    }


    pub fn create_computation_graph(&self, base_name: &str) -> Result<(), String>
    where
        Configuration: Display,
    {
        let dot_filename = format!("{}.dot", base_name);
        let png_filename = format!("{}.png", base_name);

        println!("Generating DOT file: {}", dot_filename);
        self.write_dot_file(&dot_filename)
            .map_err(|e| format!("Failed to write DOT file: {}", e))?;

        println!("Generating PNG image: {}", png_filename);
        self.generate_image(&dot_filename, &png_filename, "png")?;

        println!("Successfully created visualization files:");
        println!("  - {}", dot_filename);
        println!("  - {}", png_filename);

        Ok(())
    }

}



impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("");
        let mut count = 0;
        for (config,rule) in &self.configs {
            result.push_str(format!("{}\n", config).as_str());

            if count < self.configs.len()-1 {
                result.push_str("\t | \n");
                result.push_str(format!("\t | {}\n",rule).as_str());
                result.push_str("\t\\|/ \n");
            }
           count += 1;
        }

        write!(f, "{}", result)
    }
}