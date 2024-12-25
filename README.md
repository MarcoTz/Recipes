# Recipes

My personal recipe list, published to [Github pages](https://marcotz.github.io/Recipes)

## Format 

Recipes are written in markdown in the `Recipes` folder.
The parser expects the following format for recipes 

```
# %name

## Ingredients

* %ingredient1
* %ingredient2
...

## Steps

1. %step1
2. %step2
 
## notes 

%notes

## tags

%tags
```

Additionally, inside the separate parts, subheadings can be used, for example `### Sauce`, `### Dough`, etc.

In order to correctly parse units, each ingredient needs an amount (float) followed by a unit. Anything witout a unit, for example  `1 egg` requires an underscore to indicate no unit (instead of trying to parse `egg` as unit), i.e. `1_ egg`.
Ranges of amounts such as `1-2_ eggs` are also allowed.
