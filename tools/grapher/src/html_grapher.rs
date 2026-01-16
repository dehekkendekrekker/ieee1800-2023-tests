use crate::paths::PathMap;
use crate::paths::Node as PathNode;
use std::collections::HashMap;

pub struct HtmlRailroadGenerator {
    map_: PathMap,
}

impl HtmlRailroadGenerator {
    pub fn new(map: PathMap) -> Self {
        HtmlRailroadGenerator { map_: map }
    }

    /// Generate a complete HTML page with interactive railroad diagram
    pub fn generate_html(&self) -> String {
        let entry_point = self.map_.get_entry_point().expect("entry_point required");
        let root_node = self.map_.get_rule(entry_point.clone());

        let diagram_html = self.render_node(&root_node);
        let rules_json = self.generate_rules_json();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{entry_point} - Railroad Diagram</title>
    <style>
{CSS}
    </style>
</head>
<body>
    <h1>{entry_point}</h1>
    <div class="railroad">
        <div class="start"></div>
        {diagram_html}
        <div class="end"></div>
    </div>

    <script>
const RULES = {rules_json};

{JAVASCRIPT}
    </script>
</body>
</html>"#,
            entry_point = entry_point,
            diagram_html = diagram_html,
            rules_json = rules_json,
            CSS = CSS,
            JAVASCRIPT = JAVASCRIPT
        )
    }

    /// Generate a complete HTML page with ALL rules as railroad diagrams
    pub fn generate_html_all_rules(&self) -> String {
        let rules = self.map_.get_rules();

        // Sort rule names alphabetically
        let mut rule_names: Vec<&String> = rules.keys().collect();
        rule_names.sort();

        // Generate table of contents and diagrams
        let mut toc_html = String::new();
        let mut diagrams_html = String::new();

        for name in &rule_names {
            // Table of contents entry
            toc_html.push_str(&format!(
                "<li><a data-target=\"rule-{}\">{}</a></li>",
                html_escape(name),
                html_escape(name)
            ));

            // Diagram
            let node = rules.get(*name).unwrap();
            let diagram = self.render_node(node);
            diagrams_html.push_str(&format!(
                "<div class=\"rule-section\" id=\"rule-{}\">\n\
                    <h2>{}</h2>\n\
                    <div class=\"railroad\">\n\
                        <div class=\"start\"></div>\n\
                        {}\n\
                        <div class=\"end\"></div>\n\
                    </div>\n\
                </div>\n",
                html_escape(name),
                html_escape(name),
                diagram
            ));
        }

        let rules_json = self.generate_rules_json();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Grammar Railroad Diagrams</title>
    <style>
{CSS}
{ALL_RULES_CSS}
    </style>
</head>
<body>
    <div class="sidebar">
        <details class="legend" open>
            <summary>Legend &amp; Controls</summary>
            <div class="legend-content">
                <div class="legend-section">
                    <h4>Mouse Controls</h4>
                    <div class="legend-item"><strong>Left-click</strong> non-terminal: Expand inline</div>
                    <div class="legend-item"><strong>Right-click</strong> non-terminal: Jump to definition</div>
                </div>
                <div class="legend-section">
                    <h4>Symbols</h4>
                    <div class="legend-item"><span class="terminal">literal</span> Literal text</div>
                    <div class="legend-item"><span class="terminal regex">/regex/</span> Regular expression</div>
                    <div class="legend-item"><span class="nonterminal" style="cursor:default">rule</span> Rule reference</div>
                    <div class="legend-item"><span class="legend-optional">opt</span> Optional (0 or 1)</div>
                    <div class="legend-item"><span class="legend-repetition">rep</span> Repetition (0+)</div>
                    <div class="legend-item"><span class="legend-choice">A | B</span> Choice</div>
                </div>
            </div>
        </details>
        <h2>Rules ({rule_count})</h2>
        <input type="text" id="search" placeholder="Search rules..." />
        <ul id="toc">
            {toc_html}
        </ul>
    </div>
    <div class="main-content">
        <h1>Grammar</h1>
        {diagrams_html}
    </div>

    <script>
const RULES = {rules_json};

{JAVASCRIPT}
{ALL_RULES_JAVASCRIPT}
    </script>
</body>
</html>"#,
            rule_count = rule_names.len(),
            toc_html = toc_html,
            diagrams_html = diagrams_html,
            rules_json = rules_json,
            CSS = CSS,
            ALL_RULES_CSS = ALL_RULES_CSS,
            JAVASCRIPT = JAVASCRIPT,
            ALL_RULES_JAVASCRIPT = ALL_RULES_JAVASCRIPT
        )
    }

    /// Render a node to HTML
    fn render_node(&self, node: &PathNode) -> String {
        match node {
            PathNode::Rule(label) => {
                format!(
                    r#"<span class="nonterminal" data-rule="{}">{}</span>"#,
                    html_escape(label),
                    html_escape(label)
                )
            }

            PathNode::Literal(label) => {
                format!(
                    r#"<span class="terminal">{}</span>"#,
                    html_escape(label)
                )
            }

            PathNode::RegEx(pattern) => {
                format!(
                    r#"<span class="terminal regex">/{}/</span>"#,
                    html_escape(pattern)
                )
            }

            PathNode::Alternative(alternatives) => {
                if alternatives.len() == 1 {
                    self.render_node(&alternatives[0])
                } else {
                    let items: Vec<String> = alternatives
                        .iter()
                        .map(|alt| format!(r#"<div class="choice-item">{}</div>"#, self.render_node(alt)))
                        .collect();
                    format!(r#"<div class="choice">{}</div>"#, items.join("\n"))
                }
            }

            PathNode::Sequence(nodes) => {
                if nodes.is_empty() {
                    String::new()
                } else if nodes.len() == 1 {
                    self.render_node(&nodes[0])
                } else {
                    let items: Vec<String> = nodes
                        .iter()
                        .map(|n| self.render_node(n))
                        .collect();
                    format!(r#"<div class="sequence">{}</div>"#, items.join("\n"))
                }
            }

            PathNode::Optional(inner) => {
                format!(
                    r#"<div class="optional"><div class="optional-content">{}</div></div>"#,
                    self.render_node(inner)
                )
            }

            PathNode::Repetition(inner) => {
                format!(
                    r#"<div class="repetition"><div class="repetition-content">{}</div></div>"#,
                    self.render_node(inner)
                )
            }
        }
    }

    /// Generate JSON containing all rule definitions
    fn generate_rules_json(&self) -> String {
        let rules = self.map_.get_rules();
        let mut json_map: HashMap<String, String> = HashMap::new();

        for (name, node) in rules {
            json_map.insert(name, self.render_node(&node));
        }

        serde_json::to_string(&json_map).unwrap_or_else(|_| "{}".to_string())
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

const CSS: &str = r#"
* {
    box-sizing: border-box;
}

body {
    background-color: hsl(230, 10%, 15%);
    color: hsl(230, 30%, 80%);
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    padding: 20px;
    margin: 0;
}

h1 {
    font-family: monospace;
    color: hsl(200, 60%, 70%);
    margin-bottom: 20px;
}

.railroad {
    display: flex;
    align-items: center;
    gap: 0;
    padding: 20px;
    overflow-x: auto;
}

.start, .end {
    width: 10px;
    height: 10px;
    background: hsl(200, 10%, 60%);
    border-radius: 50%;
    flex-shrink: 0;
}

.start::after, .end::before {
    content: '';
    display: inline-block;
    width: 20px;
    height: 3px;
    background: hsl(200, 10%, 60%);
    vertical-align: middle;
}

.start::after {
    margin-left: -5px;
}

.end::before {
    margin-right: -5px;
}

.terminal, .nonterminal {
    display: inline-flex;
    align-items: center;
    padding: 4px 12px;
    margin: 2px 4px;
    font-family: monospace;
    font-size: 14px;
    white-space: nowrap;
}

.terminal {
    background: hsl(230, 20%, 25%);
    border: 2px solid hsl(200, 10%, 50%);
    border-radius: 20px;
    color: hsl(230, 30%, 80%);
}

.terminal.regex {
    border-style: dashed;
    color: hsl(30, 60%, 70%);
}

.nonterminal {
    background: hsl(230, 20%, 20%);
    border: 2px solid hsl(200, 10%, 50%);
    color: hsl(200, 60%, 70%);
    font-weight: bold;
    cursor: pointer;
    transition: all 0.2s;
}

.nonterminal:hover {
    background: hsl(230, 30%, 30%);
    border-color: hsl(200, 60%, 60%);
}

.nonterminal.expanded {
    background: hsl(200, 30%, 25%);
    border-color: hsl(200, 60%, 50%);
}

.sequence {
    display: flex;
    align-items: center;
    gap: 0;
}

.choice {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    border-left: 3px solid hsl(200, 10%, 40%);
    border-right: 3px solid hsl(200, 10%, 40%);
    padding: 5px 0;
    margin: 0 4px;
    position: relative;
}

.choice::before, .choice::after {
    content: '';
    position: absolute;
    left: -3px;
    right: -3px;
    height: 3px;
    background: hsl(200, 10%, 40%);
}

.choice::before {
    top: 0;
    border-radius: 3px 3px 0 0;
}

.choice::after {
    bottom: 0;
    border-radius: 0 0 3px 3px;
}

.choice-item {
    display: flex;
    align-items: center;
    padding: 4px 8px;
    position: relative;
}

.choice-item::before {
    content: '';
    width: 8px;
    height: 3px;
    background: hsl(200, 10%, 40%);
    flex-shrink: 0;
}

.choice-item::after {
    content: '';
    width: 8px;
    height: 3px;
    background: hsl(200, 10%, 40%);
    flex-shrink: 0;
}

.optional {
    display: flex;
    align-items: center;
    position: relative;
    margin: 0 4px;
}

.optional::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 0;
    right: 0;
    height: 3px;
    background: hsl(200, 10%, 40%);
    border-radius: 3px;
    z-index: 0;
}

.optional-content {
    position: relative;
    z-index: 1;
    border: 2px dashed hsl(200, 10%, 40%);
    border-radius: 8px;
    padding: 4px;
    background: hsl(230, 10%, 15%);
}

.repetition {
    display: flex;
    align-items: center;
    position: relative;
    margin: 0 4px;
}

.repetition-content {
    position: relative;
    z-index: 1;
    border: 2px solid hsl(120, 30%, 40%);
    border-radius: 8px;
    padding: 4px;
    background: hsl(230, 10%, 15%);
}

.repetition::after {
    content: '*';
    position: absolute;
    top: -8px;
    right: -4px;
    color: hsl(120, 50%, 60%);
    font-weight: bold;
    font-size: 16px;
}

.expanded-content {
    display: flex;
    align-items: center;
    background: hsl(230, 15%, 18%);
    border: 1px solid hsl(200, 30%, 40%);
    border-radius: 8px;
    padding: 8px;
    margin: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.collapse-btn {
    background: hsl(0, 40%, 40%);
    color: white;
    border: none;
    border-radius: 50%;
    width: 20px;
    height: 20px;
    cursor: pointer;
    margin-left: 8px;
    font-size: 12px;
    line-height: 1;
}

.collapse-btn:hover {
    background: hsl(0, 50%, 50%);
}
"#;

const JAVASCRIPT: &str = r#"
document.addEventListener('DOMContentLoaded', function() {
    document.body.addEventListener('click', function(e) {
        const nonterminal = e.target.closest('.nonterminal');
        if (!nonterminal) return;

        // Check if already expanded
        if (nonterminal.classList.contains('expanded')) {
            return; // Let collapse button handle it
        }

        const ruleName = nonterminal.dataset.rule;
        if (!ruleName || !RULES[ruleName]) return;

        // Mark as expanded
        nonterminal.classList.add('expanded');

        // Create expanded content
        const expandedDiv = document.createElement('div');
        expandedDiv.className = 'expanded-content';
        expandedDiv.innerHTML = RULES[ruleName];

        // Add collapse button
        const collapseBtn = document.createElement('button');
        collapseBtn.className = 'collapse-btn';
        collapseBtn.textContent = '×';
        collapseBtn.onclick = function(evt) {
            evt.stopPropagation();
            nonterminal.classList.remove('expanded');
            expandedDiv.remove();
        };
        expandedDiv.appendChild(collapseBtn);

        // Insert after the nonterminal
        nonterminal.insertAdjacentElement('afterend', expandedDiv);
    });
});
"#;

const ALL_RULES_CSS: &str = r#"
body {
    display: flex;
    padding: 0;
}

.sidebar {
    width: 280px;
    height: 100vh;
    position: fixed;
    left: 0;
    top: 0;
    background: hsl(230, 15%, 12%);
    border-right: 1px solid hsl(200, 10%, 30%);
    padding: 20px;
    overflow-y: auto;
    flex-shrink: 0;
    z-index: 1000;
}

.sidebar h2 {
    font-size: 16px;
    margin: 0 0 15px 0;
    color: hsl(200, 60%, 70%);
}

.sidebar input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid hsl(200, 10%, 30%);
    border-radius: 4px;
    background: hsl(230, 10%, 18%);
    color: hsl(230, 30%, 80%);
    font-size: 14px;
    margin-bottom: 15px;
}

.sidebar input:focus {
    outline: none;
    border-color: hsl(200, 60%, 50%);
}

.sidebar ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

.sidebar li {
    margin: 2px 0;
}

.sidebar a {
    display: block;
    padding: 4px 8px;
    color: hsl(230, 30%, 70%);
    text-decoration: none;
    font-family: monospace;
    font-size: 13px;
    border-radius: 4px;
    cursor: pointer;
}

.sidebar a:hover {
    background: hsl(230, 20%, 25%);
    color: hsl(200, 60%, 70%);
}

.sidebar li.hidden {
    display: none;
}

.legend {
    margin-bottom: 20px;
    border: 1px solid hsl(200, 10%, 30%);
    border-radius: 6px;
    background: hsl(230, 12%, 14%);
}

.legend summary {
    padding: 10px 12px;
    cursor: pointer;
    font-weight: bold;
    color: hsl(200, 60%, 70%);
    user-select: none;
}

.legend summary:hover {
    background: hsl(230, 15%, 18%);
}

.legend-content {
    padding: 12px;
    border-top: 1px solid hsl(200, 10%, 25%);
}

.legend-section {
    margin-bottom: 12px;
}

.legend-section:last-child {
    margin-bottom: 0;
}

.legend-section h4 {
    margin: 0 0 8px 0;
    font-size: 12px;
    color: hsl(200, 40%, 60%);
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.legend-item {
    font-size: 12px;
    margin: 6px 0;
    display: flex;
    align-items: center;
    gap: 8px;
}

.legend-item .terminal,
.legend-item .nonterminal {
    font-size: 11px;
    padding: 2px 8px;
    margin: 0;
}

.legend-optional {
    display: inline-block;
    padding: 2px 6px;
    border: 2px dashed hsl(200, 10%, 40%);
    border-radius: 6px;
    font-size: 11px;
    font-family: monospace;
    background: hsl(230, 10%, 15%);
}

.legend-repetition {
    display: inline-block;
    padding: 2px 6px;
    border: 2px solid hsl(120, 30%, 40%);
    border-radius: 6px;
    font-size: 11px;
    font-family: monospace;
    background: hsl(230, 10%, 15%);
    position: relative;
}

.legend-repetition::after {
    content: '*';
    position: absolute;
    top: -6px;
    right: -4px;
    color: hsl(120, 50%, 60%);
    font-weight: bold;
    font-size: 12px;
}

.legend-choice {
    display: inline-block;
    padding: 2px 6px;
    border-left: 2px solid hsl(200, 10%, 40%);
    border-right: 2px solid hsl(200, 10%, 40%);
    font-size: 11px;
    font-family: monospace;
}

.main-content {
    margin-left: 300px;
    padding: 20px;
    flex: 1;
}

.main-content h1 {
    margin-top: 0;
}

.rule-section {
    margin-bottom: 40px;
    padding-bottom: 20px;
    border-bottom: 1px solid hsl(200, 10%, 25%);
}

.rule-section h2 {
    font-family: monospace;
    color: hsl(200, 60%, 70%);
    margin: 0 0 10px 0;
    font-size: 18px;
}

.rule-section:target {
    background: hsl(200, 20%, 18%);
    margin: -10px;
    padding: 10px;
    padding-bottom: 30px;
    border-radius: 8px;
}

.rule-section:target h2 {
    color: hsl(200, 70%, 75%);
}
"#;

const ALL_RULES_JAVASCRIPT: &str = r#"
// Search functionality
document.getElementById('search').addEventListener('input', function(e) {
    const query = e.target.value.toLowerCase();
    const items = document.querySelectorAll('#toc li');

    items.forEach(function(item) {
        const text = item.textContent.toLowerCase();
        if (text.includes(query)) {
            item.classList.remove('hidden');
        } else {
            item.classList.add('hidden');
        }
    });
});

// Sidebar link clicks - scroll vertically only
document.querySelectorAll('.sidebar a[data-target]').forEach(function(link) {
    link.addEventListener('click', function(e) {
        const targetId = this.dataset.target;
        const section = document.getElementById(targetId);
        if (section) {
            const y = section.getBoundingClientRect().top + window.scrollY;
            window.scrollTo({ left: 0, top: y, behavior: 'smooth' });
        }
    });
});

// Right-click on nonterminal to navigate to that rule (without expanding)
document.body.addEventListener('contextmenu', function(e) {
    const nonterminal = e.target.closest('.nonterminal');
    if (!nonterminal) return;

    const ruleName = nonterminal.dataset.rule;
    if (!ruleName) return;

    // Check if this rule exists as a section
    const section = document.getElementById('rule-' + ruleName);
    if (section) {
        e.preventDefault();
        const y = section.getBoundingClientRect().top + window.scrollY;
        window.scrollTo({ left: 0, top: y, behavior: 'smooth' });
    }
});
"#;
