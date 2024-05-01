function filterRecipes() {
  recipes = document.getElementById('recipe_list').children;
  if (typeof(recipes) == 'undefined' || recipes === null) { return; } 
  search_value = document.getElementById('search').value.toLowerCase();
  if (typeof(search_value) == 'undefined' || search_value === null) { return; } 
 
  for(recipe of recipes){
    recipe_name = recipe.textContent.toLowerCase();
    recipe_tags = ""
    for(child of recipe.children) {
      if(child.className == 'recipe_taglist') { recipe_tags = child.textContent; break;}
    }

    matches_name = recipe_name.includes(search_value);
    matches_tags = recipe_tags.includes(search_value);
    
    matches = matches_name || matches_tags 
    if(matches) { recipe.style.display="block"; }
    else { recipe.style.display="none"; }
  }
}
