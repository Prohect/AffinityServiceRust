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
            if ($0 ~ /\/\/\// || $0 ~ /#/) next
            decl = decl "\n" $0
            if (is_fn && $0 ~ /\{/) {
                # Function signature complete at {
                processed = decl
                sub(/^pub /, "", processed)
                sub(/ \{.*/, "", processed)
                print "- [L" start_line ":L" NR "]" processed
                in_decl = 0
                is_fn = 0
                decl = ""
                doc_start = 0
            } else if (!is_fn && ($0 ~ /;|\}/)) {
                # For structs/enums, stop at ; or }
                processed = decl
                sub(/^pub /, "", processed)
                # Remove inline // comments
                gsub(/ \/\/[^\n]*/, "", processed)
                print "- [L" start_line ":L" NR "]" processed
                in_decl = 0
                decl = ""
                doc_start = 0
            }
        } else if (/^pub fn|^fn|^pub struct|^struct|^pub enum|^enum/) {
            in_decl = 1
            start_line = doc_start ? doc_start : NR
            decl = $0
            if (/^pub fn|^fn/) {
                is_fn = 1
                if ($0 ~ /\{/) {
                    # Function signature on single line
                    processed = $0
                    sub(/^pub /, "", processed)
                    sub(/ \{.*/, "", processed)
                    print "- [L" start_line ":L" NR "]" processed
                    in_decl = 0
                    is_fn = 0
                    decl = ""
                    doc_start = 0
                }
            } else {
                # For structs/enums, check if complete on first line
                if ((/^pub struct|^struct/ && (/;|\}/)) || (/^pub enum|^enum/ && /\}/)) {
                    processed = $0
                    sub(/^pub /, "", processed)
                    print "- [L" start_line ":L" NR "]" processed
                    in_decl = 0
                    decl = ""
                    doc_start = 0
                }
            }
        } else if (/^static/) {
            sub(/^pub /, "", $0)
            print "- [L" NR ":L" NR "]" $0
        } else if (/^pub / && !/^pub (struct|enum|fn)/) {
            sub(/^pub /, "", $0)
            print "- [L" NR ":L" NR "]" $0
        } else if ($0 ~ /^\/\/\//) {
            if (doc_start == 0) doc_start = NR
        } else if ($0 !~ /^[ \t]*$/ && $0 !~ /^\/\// && $0 !~ /^#/) {
            doc_start = 0
        }
    }
    ' "$file"

    echo ""
done
