#!/bin/bash

# Generate src code outline
# Usage: ./generate_outline.sh > outline.md

# Try to read workspace members from Cargo.toml
if [ -f "Cargo.toml" ]; then
    members=$(sed -n '/^members = \[/,/^\]/p' Cargo.toml | grep '^    "' | sed 's/.*"\([^"]*\)".*/\1/')
fi

if [ -n "$members" ]; then
    echo "# Workspace Code Structure Outline"
    echo ""
    for file in $(for member in $members; do find "$member" -name "*.rs" 2>/dev/null; done | sort); do
        echo "## ${file#crates/}"

        # Use awk to parse and accumulate multi-line declarations
        awk '
        function count_braces(line) {
            split(line, chars, "")
            for (i in chars) {
                if (chars[i] == "{") brace_count++
                else if (chars[i] == "}") brace_count--
            }
        }
        {
            if (in_decl) {
                if ($0 ~ /\/\/\// || $0 ~ /#/) next
                decl = decl "\n" $0
                count_braces($0)
                if (is_fn) {
                    if (brace_count == 0 && initial_brace_found) {
                        # Function ends when braces balance
                        processed = decl
                        sub(/^pub /, "", processed)
                        sub(/ \{.*/, "", processed)
                        print "- [L" start_line ":L" NR "]" processed
                        in_decl = 0
                        is_fn = 0
                        decl = ""
                        doc_start = 0
                        brace_count = 0
                        initial_brace_found = 0
                    } else if ($0 ~ /\{/ && !initial_brace_found) {
                        initial_brace_found = 1
                    }
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
                brace_count = 0
                initial_brace_found = 0
                if (/^pub fn|^fn/) {
                    is_fn = 1
                    count_braces($0)
                    if ($0 ~ /\{/) {
                        initial_brace_found = 1
                    }
                    if (brace_count == 0 && initial_brace_found) {
                        # Single-line function
                        processed = $0
                        sub(/^pub /, "", processed)
                        sub(/ \{.*/, "", processed)
                        print "- [L" start_line ":L" NR "]" processed
                        in_decl = 0
                        is_fn = 0
                        decl = ""
                        doc_start = 0
                        brace_count = 0
                        initial_brace_found = 0
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
else
    echo "# Src Code Structure Outline"
    echo ""
    for file in src/*.rs; do
        echo "## ${file#src/}"

        # Use awk to parse and accumulate multi-line declarations
        awk '
        function count_braces(line) {
            split(line, chars, "")
            for (i in chars) {
                if (chars[i] == "{") brace_count++
                else if (chars[i] == "}") brace_count--
            }
        }
        {
            if (in_decl) {
                if ($0 ~ /\/\/\// || $0 ~ /#/) next
                decl = decl "\n" $0
                count_braces($0)
                if (is_fn) {
                    if (brace_count == 0 && initial_brace_found) {
                        # Function ends when braces balance
                        processed = decl
                        sub(/^pub /, "", processed)
                        sub(/ \{.*/, "", processed)
                        print "- [L" start_line ":L" NR "]" processed
                        in_decl = 0
                        is_fn = 0
                        decl = ""
                        doc_start = 0
                        brace_count = 0
                        initial_brace_found = 0
                    } else if ($0 ~ /\{/ && !initial_brace_found) {
                        initial_brace_found = 1
                    }
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
                brace_count = 0
                initial_brace_found = 0
                if (/^pub fn|^fn/) {
                    is_fn = 1
                    count_braces($0)
                    if ($0 ~ /\{/) {
                        initial_brace_found = 1
                    }
                    if (brace_count == 0 && initial_brace_found) {
                        # Single-line function
                        processed = $0
                        sub(/^pub /, "", processed)
                        sub(/ \{.*/, "", processed)
                        print "- [L" start_line ":L" NR "]" processed
                        in_decl = 0
                        is_fn = 0
                        decl = ""
                        doc_start = 0
                        brace_count = 0
                        initial_brace_found = 0
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
fi
