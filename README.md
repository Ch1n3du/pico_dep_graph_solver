# pico_dep_graph_solver

This is a simple dependency graph solver based on a question I saw on HackerRank.

## Usage

---

The CLI takes two arguments:

### 1. The Tasks File

This is a JSON file containing the tasks in the following shema.

```json
[
    {
        "name": "A",
        "files": [
            "about.md",
            "001.py"
        ],
        "deps": []
    },
    {
        "name": "B",
        "files": [],
        "deps": [
            "A"
        ]
    }
]

```

### 2. The Changed Tasks file

This is a JSON file containing the files changed as a list of strings.

```json
[
    "about.md",
    "001.py",
    "002.py"
]
```
