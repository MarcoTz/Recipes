import requests
import os
import recipe_scrapers

recipe_template : str = '''
# %s

## Ingredients 
%s 

## Steps
%s 

## Tags
%s 
'''

recipes_dir : str = 'Recipes'

def get_recipe_scraper(url:str):
    recipe_html = requests.get(url).content.decode('utf-8')
    return recipe_scrapers.scrape_html(html=recipe_html,org_url=url)

def get_recipe_md(scraper) -> tuple[str,str]:

    recipe_name         : str = scraper.title()
    recipe_ingredients  : list[str] = scraper.ingredients()
    recipe_ingredients_str : str = ', '.join(recipe_ingredients)
    recipe_steps        : str = scraper.instructions()
    recipe_tags         : list[str] = scraper.keywords()
    recipe_tags_str     : str = ', '.join(recipe_tags)

    recipe_md : str = recipe_template % (recipe_name,recipe_ingredients_str,recipe_steps,recipe_tags_str)
    return (recipe_name,recipe_md)

def save_recipe(recipe_title:str,recipe_md:str) -> None:
    recipe_out_name = recipe_title.replace(' ','')+'.md'
    recipe_out_path = os.path.join(recipes_dir,recipe_out_name)

    if os.path.exists(recipe_out_path):
        overwrite_answer : str = input('Recipe %s already exists, overwrite? (Y/N)')
        if overwrite_answer.lower() != 'y':
            return

    recipe_out_file = open(recipe_out_path,'w')
    recipe_out_file.write(recipe_md)
    recipe_out_file.close()
    print('Wrote recipe to %s' % recipe_out_path) 


if __name__ == '__main__':
    recipe_url = input('Please enter recipe URL: ')
    recipe_scraper = get_recipe_scraper(recipe_url)
    (recipe_title,recipe_md) = get_recipe_md(recipe_scraper)
    save_recipe(recipe_title,recipe_md)

