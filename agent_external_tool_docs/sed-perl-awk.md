# sed / perl / awk (via Git for Windows)

**Purpose:** Regex find/replace for text files  
**Source:** Bundled with Git for Windows (available in Git Bash or any shell after Git install)

## sed (Stream Editor)

```sh
# Find lines matching pattern
sed -n '/pattern/p' file.txt

# Replace in-place
sed -i 's/old/new/g' file.txt

# Replace with capture groups
sed -i 's/^\(.*\.exe,.*\)$/\1,suffix/' file.txt

# Delete lines matching pattern
sed -i '/pattern/d' file.txt
```

## perl (One-liner regex)

```sh
# Replace in-place (best regex support)
perl -i -pe 's/old/new/g' file.txt

# With UTF-8 support
perl -i -CSD -pe 's/old/new/g' file.txt

# Multi-line patterns
perl -i -0pe 's/start.*?end/replacement/gs' file.txt
```

## awk (Pattern processing)

```sh
# Print lines matching pattern
awk '/pattern/' file.txt

# Replace and print
awk '{gsub(/old/, "new"); print}' file.txt > output.txt

# Conditional processing
awk -F',' '/\.exe,/ {print $0 ",testNone"; next} {print}' file.txt
```

## Notes

- Available via Git Bash (included with Git for Windows)
- `sed -i` modifies files in-place
- `perl` has the most powerful regex (PCRE)
- Prefer these over PowerShell for complex regex (avoids escaping issues)
