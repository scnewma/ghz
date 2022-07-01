# ghz

Github repository fuzzy filter that caches repository lists locally for fast searching.

## Prerequisites
- [gh](https://cli.github.com/)

## Usage

Create a configuration file at `$HOME/.config/ghz/config.json` to state the owners of the repositories that you want to store in the cache.

```
{
    "owners": [
        {"name": "scnewma"},
        {
            "name": "hashicorp",
            "limit": 5000,
            "filter_on": "Name",
            "mappings": {
                "Title": "Name",
                "Match": "Name"
            }
        }
    ]
}
```

Refresh the cache anytime you want to check for new repositories.

```
ghz refresh
```

Filter the repositories.

```
ghz filter <FILTER>
```

If you pass the `--alfred` flag `ghz` will output the results in alfred script filter format which can be easily hooked into an Alfred workflow.

```
ghz filter --alfred <FILTER>
```
