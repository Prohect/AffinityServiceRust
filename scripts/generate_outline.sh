#!/bin/bash

# Generate src code outline
# Usage: ./generate_outline.sh > <output_file>
# eg: ./generate_outline.sh > outline.md

# Try to read workspace members from Cargo.toml
if [ -f "Cargo.toml" ]; then
    members=$(sed -n '/^members = \[/,/^\]/p' Cargo.toml | grep '^    "' | sed 's/.*"\([^"]*\)".*/\1/')
fi

if [ -n "$members" ]; then
    echo "# Workspace Code Structure Outline, **READ this by MULTIPLE calls if it's too large being outlined by first call**"
    files=$(for member in $members; do find "$member" -name "*.rs" 2>/dev/null; done | sort)
else
    echo "# Src Outline, **READ this by MULTIPLE calls if it's too large being outlined by first call**"
    files=$(echo src/*.rs)
fi

echo ""

for file in $files; do
    echo "## $file"

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
                    if (in_impl_fn) sub(/^    /, "", processed)
                    sub(/^pub /, "", processed)
                    sub(/\{.*/, "", processed)
                    # Trim trailing whitespace
                    gsub(/[ \t]+\n/, "\n", processed)
                    # Remove lines with only whitespace
                    gsub(/\n[ \t]*\n/, "\n", processed)
                    # Remove blank lines and extra newlines
                    gsub(/\n\n+/, "\n", processed)
                    gsub(/^\n+/, "", processed)
                    gsub(/\n+$/, "", processed)
                    if (in_impl_fn)
                        print "  - [L" start_line ":" NR "]" processed
                    else
                        print "- [L" start_line ":" NR "]" processed
                    in_decl = 0
                    is_fn = 0
                    in_impl_fn = 0
                    decl = ""
                    doc_start = 0
                    brace_count = 0
                    initial_brace_found = 0
                } else if ($0 ~ /\{/ && !initial_brace_found) {
                    initial_brace_found = 1
                }
            } else if (is_struct_enum) {
                if ((!initial_brace_found && $0 ~ /;/) || (initial_brace_found && $0 ~ /}/ && brace_count == 0)) {
                    # Struct/enum ends at ; (no brace) or balanced }
                    processed = decl
                    sub(/^pub /, "", processed)
                    # Remove inline // comments
                    gsub(/ \/\/[^\n]*/, "", processed)
                    # Trim trailing whitespace
                    gsub(/[ \t]+\n/, "\n", processed)
                    # Remove lines with only whitespace
                    gsub(/\n[ \t]*\n/, "\n", processed)
                    # Remove blank lines and extra newlines
                    gsub(/\n\n+/, "\n", processed)
                    gsub(/^\n+/, "", processed)
                    gsub(/\n+$/, "", processed)
                    print "- [L" start_line ":" NR "]" processed
                    in_decl = 0
                    is_struct_enum = 0
                    decl = ""
                    doc_start = 0
                    brace_count = 0
                    initial_brace_found = 0
                } else if ($0 ~ /\{/ && !initial_brace_found) {
                    initial_brace_found = 1
                }
            } else if (is_impl_header) {
                # Accumulating multi-line impl signature (where clause etc.)
                if ($0 ~ /\{/) {
                    processed = decl
                    sub(/ *\{.*/, "", processed)
                    gsub(/[ \t]+\n/, "\n", processed)
                    gsub(/\n[ \t]*\n/, "\n", processed)
                    gsub(/\n\n+/, "\n", processed)
                    gsub(/^\n+/, "", processed)
                    gsub(/\n+$/, "", processed)
                    print "- [L" start_line ":" NR "]" processed
                    in_decl = 0
                    is_impl_header = 0
                    in_impl = 1
                    decl = ""
                    doc_start = 0
                    brace_count = 0
                }
            }
        } else if (/^pub fn |^fn /) {
            # Top-level function
            in_decl = 1
            start_line = doc_start ? doc_start : NR
            decl = $0
            brace_count = 0
            initial_brace_found = 0
            is_fn = 1
            is_struct_enum = 0
            in_impl_fn = 0
            is_impl_header = 0
            count_braces($0)
            if ($0 ~ /\{/) {
                initial_brace_found = 1
            }
            if (brace_count == 0 && initial_brace_found) {
                # Single-line function
                processed = $0
                sub(/^pub /, "", processed)
                sub(/\{.*/, "", processed)
                gsub(/[ \t]+\n/, "\n", processed)
                gsub(/\n[ \t]*\n/, "\n", processed)
                gsub(/\n\n+/, "\n", processed)
                gsub(/^\n+/, "", processed)
                gsub(/\n+$/, "", processed)
                print "- [L" start_line ":" NR "]" processed
                in_decl = 0
                is_fn = 0
                decl = ""
                doc_start = 0
                brace_count = 0
                initial_brace_found = 0
            }
        } else if (/^pub struct |^struct |^pub enum |^enum /) {
            # Top-level struct/enum
            in_decl = 1
            start_line = doc_start ? doc_start : NR
            decl = $0
            brace_count = 0
            initial_brace_found = 0
            is_fn = 0
            is_struct_enum = 1
            is_impl_header = 0
            count_braces($0)
            has_brace = ($0 ~ /\{/)
            # FIX: Only check ; when no brace on this line
            if ((!has_brace && $0 ~ /;/) || (has_brace && brace_count == 0)) {
                # Single-line struct/enum
                processed = $0
                sub(/^pub /, "", processed)
                gsub(/ \/\/[^\n]*/, "", processed)
                gsub(/[ \t]+\n/, "\n", processed)
                gsub(/\n[ \t]*\n/, "\n", processed)
                gsub(/\n\n+/, "\n", processed)
                gsub(/^\n+/, "", processed)
                gsub(/\n+$/, "", processed)
                print "- [L" start_line ":" NR "]" processed
                in_decl = 0
                is_struct_enum = 0
                decl = ""
                doc_start = 0
            } else {
                if ($0 ~ /\{/) initial_brace_found = 1
            }
        } else if (/^impl[ <]/) {
            # Start of impl block
            start_line = doc_start ? doc_start : NR
            doc_start = 0
            if ($0 ~ /\{/) {
                # Single-line impl header (most common: impl Foo {)
                processed = $0
                sub(/ *\{.*/, "", processed)
                print "- [L" NR "]" processed
                in_impl = 1
            } else {
                # Multi-line impl header (where clause etc.)
                in_decl = 1
                is_impl_header = 1
                is_fn = 0
                is_struct_enum = 0
                decl = $0
                brace_count = 0
            }
        } else if (in_impl && ($0 ~ /^    pub fn / || $0 ~ /^    fn /)) {
            # Function inside impl block
            in_decl = 1
            start_line = doc_start ? doc_start : NR
            decl = $0
            brace_count = 0
            initial_brace_found = 0
            is_fn = 1
            is_struct_enum = 0
            in_impl_fn = 1
            is_impl_header = 0
            count_braces($0)
            if ($0 ~ /\{/) {
                initial_brace_found = 1
            }
            if (brace_count == 0 && initial_brace_found) {
                # Single-line function in impl
                processed = $0
                sub(/^    /, "", processed)
                sub(/^pub /, "", processed)
                sub(/\{.*/, "", processed)
                gsub(/[ \t]+\n/, "\n", processed)
                gsub(/\n[ \t]*\n/, "\n", processed)
                gsub(/\n\n+/, "\n", processed)
                gsub(/^\n+/, "", processed)
                gsub(/\n+$/, "", processed)
                print "  - [L" start_line ":" NR "]" processed
                in_decl = 0
                is_fn = 0
                in_impl_fn = 0
                decl = ""
                doc_start = 0
                brace_count = 0
                initial_brace_found = 0
            }
        } else if (in_impl && /^\}/) {
            # End of impl block
            in_impl = 0
            doc_start = 0
        } else if (/^static/) {
            sub(/^pub /, "", $0)
            print "- [L" NR ":" NR "]" $0
        } else if (/^pub / && !/^pub (struct|enum|fn|use)/) {
            sub(/^pub /, "", $0)
            print "- [L" NR ":" NR "]" $0
        } else if ($0 ~ /^[ \t]*\/\/\//) {
            # Doc comment (top-level or indented in impl)
            if (doc_start == 0) doc_start = NR
        } else if ($0 !~ /^[ \t]*$/ && $0 !~ /^[ \t]*\/\// && $0 !~ /^[ \t]*#/) {
            doc_start = 0
        }
    }
    ' "$file"

    echo ""
done
