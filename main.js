function filterRecipes() {
  recipes = document.getElementById('recipe_list').children;
  search_value = document.getElementById('search').value.toLowerCase();
  
  for(recipe of recipes){
    recipe_name = recipe.textContent.toLowerCase();
    if(!recipe_name.includes(search_value)){
      recipe.style.display = "none";
    } else { recipe.style.display = "block"; }
  }
}
