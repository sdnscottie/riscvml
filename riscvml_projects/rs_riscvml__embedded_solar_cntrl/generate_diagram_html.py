#!/usr/bin/env python3
"""
Generate HTML files that embed draw.io diagrams using viewer-static.min.js
with inline XML in the data-mxgraph attribute.

This approach works with file:// protocol since it doesn't rely on fetch().
"""

import json
import html
import os


def generate_html(drawio_path: str, title: str, header_h1: str, header_p: str) -> str:
    """Read a .drawio file and generate an HTML file with inline XML embedding."""
    with open(drawio_path, "r", encoding="utf-8") as f:
        xml_content = f.read()

    # Build the JSON object for data-mxgraph.
    # The XML goes into the "xml" key as a string value.
    mxgraph_obj = {
        "highlight": "#0000ff",
        "nav": True,
        "resize": True,
        "toolbar": "pages zoom layers tags lightbox",
        "xml": xml_content,
    }

    # Serialize to JSON (compact, no extra whitespace).
    mxgraph_json = json.dumps(mxgraph_obj, ensure_ascii=False)

    # Escape for use inside an HTML attribute value (single-quoted).
    # We need to escape: & < > ' "
    # html.escape handles &, <, >, " — we also need to handle single quotes
    # since the attribute is single-quoted.
    mxgraph_attr = html.escape(mxgraph_json, quote=True)
    # html.escape with quote=True escapes " to &quot; but not '.
    # Since our attribute uses single quotes, escape ' to &#39;.
    mxgraph_attr = mxgraph_attr.replace("'", "&#39;")

    html_content = f'''<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{html.escape(title)}</title>
<style>
  body {{ font-family: 'Segoe UI', system-ui, sans-serif; margin: 0; padding: 0; background: #FAFAFA; }}
  header {{ background: #0D47A1; color: #fff; padding: 1rem 2rem; }}
  header h1 {{ font-size: 1.4rem; font-weight: 600; margin: 0; }}
  header p {{ font-size: 0.85rem; opacity: 0.85; margin: 0.25rem 0 0; }}
  .diagram-container {{ width: 100%; padding: 1rem; background: #fff; }}
</style>
</head>
<body>
<header>
  <h1>{html.escape(header_h1)}</h1>
  <p>{html.escape(header_p)}</p>
</header>
<div class="diagram-container">
  <div class="mxgraph" data-mxgraph='{mxgraph_attr}'></div>
</div>
<script type="text/javascript" src="https://viewer.diagrams.net/js/viewer-static.min.js"></script>
</body>
</html>'''

    return html_content


def main():
    base = os.path.dirname(os.path.abspath(__file__))

    configs = [
        {
            "drawio": os.path.join(
                base,
                "rs_riscvml_embed__solar_cntrl__full",
                "docs_about__rs_riscvml_embed__solar_cntrl__full",
                "wintergarden-solar-array.drawio",
            ),
            "title": "rs_riscvml_embed_solar_cntrl \u2014 Wintergarden Solar Array (Full Reference)",
            "h1": "rs_riscvml_embed_solar_cntrl \u2014 Wintergarden Solar Array",
            "p": "Full Reference Solution | 12x TrinaSolar TSM-620NEG19RC.20 | ESP32-P4 Controller",
        },
        {
            "drawio": os.path.join(
                base,
                "rs_riscvml_embed__solar_cntrl__free",
                "docs_about__rs_riscvml_embed__solar_cntrl__free",
                "wintergarden-solar-array.drawio",
            ),
            "title": "rs_riscvml_embed_solar_cntrl \u2014 Wintergarden Solar Array (Free Edition)",
            "h1": "rs_riscvml_embed_solar_cntrl \u2014 Wintergarden Solar Array",
            "p": "Free Edition (Student Exercise) | 12x TrinaSolar TSM-620NEG19RC.20 | ESP32-P4 Controller",
        },
    ]

    for cfg in configs:
        drawio_path = cfg["drawio"]
        if not os.path.isfile(drawio_path):
            print(f"ERROR: drawio file not found: {drawio_path}")
            continue

        html_content = generate_html(drawio_path, cfg["title"], cfg["h1"], cfg["p"])

        # Write HTML alongside the drawio file
        output_path = drawio_path.replace(".drawio", ".html")
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(html_content)

        size_kb = os.path.getsize(output_path) / 1024
        print(f"OK: {output_path} ({size_kb:.1f} KB)")


if __name__ == "__main__":
    main()
