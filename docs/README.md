# Docs

the docs of this project follows a MSDN style doc schema.
that is: 
each source file is a dir(/<$locale>/<$source_file_name>/, eg. "/en-US/config.rs/") with a README.md which contains an overview that follows [`schema`](#schema-for-overview-for-a-souce-file).
each top level item of a souce file is a markdown in that dir(eg. "\en-US\config.rs\read_config.md") that follows [`schema`](#schema-for-top-level-item-for-a-souce-file).
naming follows clippy. Funcs of structs or enums have a doc file name like CamelCaseName_snake_name.md

## Schema for Overview for a Souce File
| Section | Description |
|---------|-------------|
| **Title** | `# ModuleName type (<$project_name>)` |
| **Short description** | One-paragraph summary |
| **<$item_type>** | table for items that is this type |
| **See Also** | table for see-alsos |

## Schema for Top Level Item for a Souce File

| Section | Description |
|---------|-------------|
| **Title** | `# ItemName type (<$source_file_name>)` |
| **Short description** | One-paragraph summary |
| **Syntax** | Rust code block with full signature |
| **Parameters** | Per-parameter description (functions) |
| **Members** | Per-field description (structs/enums) |
| **Return value** | What the function returns |
| **Remarks** | Algorithms, improtant side effect, examples, edge cases, platform notes |
| **Requirements** | Table of module, callers, callees, API, privileges |
| **See Also** | table for see-alsos |

## Cross-references between items use relative markdown links

[ProcessConfig](en-US/config.rs/ProcessConfig.md)

[return value for ProcessSnapshot::take()](en-US/process.rs/ProcessSnapshot.md#return-value)

[log_error_if_new](../docs/en-US/apply.rs/log_error_if_new.md)

## Locales

| Locale | Overview |
|--------|----------|
| en-US | [en-US/README.md](en-US/README.md) |
| zh-CN | [zh-CN/README.md](zh-CN/README.md) |

## CONTRIBUTING

Do not read docs until you needs to update them, prefer read source to understand the implements.
Know enough about this project before documenting.
The docs should only contain the information of the project, no history needs to be documented.
Report any code issue to the leader/caller if the issue is essential.
Update en-US first, do not considering update for all locales at same time.
Cross check terms between docs to make sure they use same terms after batch documenting.
Translate to other locales after documenting to en-US done. 

## Documentation on Commit SHA

Always leave a git commit SHA as a clickable link at the bottom of every doc file.

## Current commit
*Documented for Commit: [facc6e1](https://github.com/Prohect/cs2UtilityEndPositionCacheServer/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
