#!/bin/bash

# Generate src code outline for AffinityServiceRust
# Usage: ./generate_outline.sh > src_outline.md

echo "# AffinityServiceRust Src Code Structure Outline"
echo ""

for file in src/*.rs; do
    if [[ "$file" == "src/index.crates.io" ]]; then continue; fi  # Skip symlink dir
    echo "## ${file#src/}"

    # Use awk to parse and accumulate multi-line declarations
    awk '
    {
        if (in_decl) {
            if ($0 ~ /\/\/\// || $0 ~ /\/\// || $0 ~ /#/) next
            decl = decl "\n" $0
            if ($0 ~ /\{|;|\}/) {
                # Process the accumulated declaration
                processed = decl
                # For functions, remove everything after the opening brace
                if (processed ~ /^fn/) {
                    sub(/ \{.*/, "", processed)
                }
                # Remove pub prefix
                sub(/^pub /, "", processed)
                print "- " processed
                in_decl = 0
                decl = ""
            }
        } else if (/^pub fn|^fn|^pub struct|^struct|^pub enum|^enum/) {
            in_decl = 1
            decl = $0
            # For enums, always accumulate; for others, process immediately if complete
            if ((/^pub fn|^fn/ && /\{/) || (/^pub struct|^struct/ && (/;|\}/))) {
                processed = $0
                if (processed ~ /^pub fn|^fn/) {
                    sub(/ \{.*/, "", processed)
                }
                sub(/^pub /, "", processed)
                print "- " processed
                in_decl = 0
                decl = ""
            }
        } else if (/^static/) {
            sub(/^pub /, "", $0)
            print "- " $0
        } else if (/^pub / && !/^pub (struct|enum|fn)/) {
            sub(/^pub /, "", $0)
            print "- " $0
        }
    }
    ' "$file"

    echo ""
done
